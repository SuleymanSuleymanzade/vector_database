use crate::algorithms::fast_search as fast_search;
use crate::vector_db;
use std::collections::HashMap;
use super::fast_search::FastSearch;
use serde::{Serialize, Deserialize, Deserializer};
use serde_json;
//use serde::de::{self, Visitor, MapAccess};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub enum KDTreeNode {
    Leaf(vector_db::Vector),
    Internal {
        left: Box<KDTreeNode>,
        right: Box<KDTreeNode>,
        split_value: f32,
        split_dimension: usize,
    },
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[allow(dead_code)]
pub struct KDTree{
    root: KDTreeNode,
    state: HashMap<String, usize>
}

impl KDTree{

    pub fn new(points: Vec<vector_db::Vector>, state: HashMap<String, usize>) -> Self{
        //Constructor    
        if !state.contains_key("depth"){
            panic!("There is not 'depth' uint parameter in passed state.");
        }
        let temp_root: KDTreeNode = KDTree::build(points, *state.get("depth").unwrap());
        Self{
            root:temp_root,
            state: state
        }
    }

    pub fn build(points: Vec<vector_db::Vector>, depth: usize) -> KDTreeNode {
        if points.len() == 1 {
            return KDTreeNode::Leaf(points[0]);
        }

        let dim = depth % 3; // K is the number of dimensions
        let mut sorted_points = points;
        sorted_points.sort_by(|a, b| a[dim].partial_cmp(&b[dim]).unwrap());
        let median_idx = sorted_points.len() / 2;
        let median_value = sorted_points[median_idx][dim];
        
        KDTreeNode::Internal {
            left: Box::new(
                KDTree::build(sorted_points[..median_idx].to_vec(), 
                depth + 1)),
            
            right: Box::new(
                KDTree::build(sorted_points[median_idx..].to_vec(), 
                depth + 1)),
            
            split_value: median_value,
            split_dimension: dim,
        }
    }

    pub fn nearest<'b>(
        &'b self, 
        query: &vector_db::Vector,
        node: &'b KDTreeNode, 
        best: Option<&'b vector_db::Vector>, 
        best_dist: f32) -> Option<&'b vector_db::Vector> {
            match node {
                KDTreeNode::Leaf(point) => {
                    let dist = vector_db::VectorDB::euclidean_distance(query, point);
                    if dist < best_dist {
                        Some(point)
                    } else {
                        best
                    }
                },
                KDTreeNode::Internal { left, right, split_value, split_dimension } => {
                    let next_node = if query[*split_dimension] < *split_value { left } else { right };
                    let other_node = if query[*split_dimension] < *split_value { right } else { left };
                    let updated_best = self.nearest(query, next_node, best, best_dist);
                    let updated_best_dist = updated_best.map_or(best_dist, |v| vector_db::VectorDB::euclidean_distance(query, v));
                    if (query[*split_dimension] - split_value).abs() < updated_best_dist {
                        self.nearest(query, other_node, updated_best, updated_best_dist)
                    } else {
                        updated_best
                    }
                }
            }
        }

    fn insert_recursive(&self, node: KDTreeNode, point: vector_db::Vector, depth: usize) -> KDTreeNode {
        match node {
            KDTreeNode::Leaf(_) => {
                let mut points = Vec::new();
                if let KDTreeNode::Leaf(existing_point) = node {
                    points.push(existing_point);
                }
                points.push(point);
                KDTree::build(points, depth)
            }
            KDTreeNode::Internal { mut left, mut right, split_value, split_dimension } => {
                let dim = depth % 3;
                if point[dim] < split_value {
                    left = Box::new(self.insert_recursive(*left, point, depth + 1));
                } else {
                    right = Box::new(self.insert_recursive(*right, point, depth + 1));
                }
                KDTreeNode::Internal {
                    left,
                    right,
                    split_value,
                    split_dimension,
                }
            }
        }
    }
    pub fn insert(&mut self, point: vector_db::Vector) {
            self.root = self.insert_recursive(self.root.clone(), point, *self.state.get("depth").unwrap());
    }
}

impl FastSearch for KDTree{
    fn nearest_neighbor(&self, query: &vector_db::Vector) -> Option<&vector_db::Vector> {
        self.nearest(query, &self.root, None, f32::MAX)
    }

    fn get_state(&self) -> serde_json::Result<String>{
        serde_json::to_string(&self)
    }

    fn set_state(&mut self, serialized_state: &str)->serde_json::Result<()>{
        let unserialized_state: KDTree = serde_json::from_str(serialized_state)?;
        *self = unserialized_state;
        Ok(())
    }
}