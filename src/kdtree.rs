use crate::vector_db;
use std::collections::HashMap;




#[derive(Debug)]
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

#[derive(Debug)]
#[allow(dead_code)]
pub struct KDTree {
    root: KDTreeNode,
}

impl KDTree {

    pub fn new(points: Vec<vector_db::Vector>, state: HashMap<&str, usize>) -> Self{
        
        if !state.contains_key("depth"){
            panic!("There is not 'depth' uint parameter in passed state.");
        }
        let temp_root: KDTreeNode = KDTree::build(points, *state.get("depth").unwrap());
        Self{
            root:temp_root
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

    pub fn nearest_neighbor(&self, query: &vector_db::Vector) -> Option<&vector_db::Vector> {
        self.nearest(query, &self.root, None, f32::MAX)
    }

    pub fn nearest<'a>(
        &'a self, 
        query: &vector_db::Vector,
        node: &'a KDTreeNode, 
        best: Option<&'a vector_db::Vector>, 
        best_dist: f32) -> Option<&'a vector_db::Vector> {
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
}