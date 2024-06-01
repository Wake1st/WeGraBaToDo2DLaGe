/// 1.0-  set graph params (number of levels, weighted-growth-exponent, min/max lowest level weight)
/// 1.1-  rand lowest level weight
/// 1.2-  iterate to highest number
/// 2.0-  set node-growth-exponent (inverse of the weighted-growth-exponent?)
/// 2.1-  create initial, highest, node
/// 2.2-  iterate down, setting more nodes per level
/// 3.0-  set expansion parameters (the min/max angle from the parent branch, and the rate at which those angles increase per level)
/// 3.1-  place initial node
/// 3.2-  iterate outward, based on the expansion params
/// 4.0-  set landtype params (types per height and gradient [determined based on surrounding heights])
/// 4.1-  create hightmap
/// 4.2-  asses gradients
/// 5.0-  setup sprites and road incline params [how high a road can climb]
/// 5.1-  fill in sprite data based on height and gradient
/// 5.2-  draw roads [connect nodes based on incline params - create the shortest, tolerable road]
/// 5.3-  add civic land based on surrounding nodes
/// 6.0-  building placement params (node-neighbor-weights, cell node weights)
/// 6.1-  place harvestor structures (node weight, connecting nodes)
/// 6.2-  place buildings [based on surrounding nodes in cell]
/// 6.3-  place filler structures (walls, wells)
/// 7.0-  needs params for unique area placement
/// 7.1-  setup unique areas (toad hut [should be most removed from other civic structures], fishing dock [cliffside, near highest node])
/// 8.0-  shoreline params, river dead end params
/// 8.1-  draw the shoreline
/// 8.2-  draw ponds, lakes
/// 8.3-  draw rivers [lowest crevasse] (ensure rivers spill into the sea - some can dissapear into the land)
use bevy::prelude::*;

const MAX_LAYERS: i8 = 5;

pub struct WeightedNode {
    pub weight: f32,
    pub location: Vec2,
}

fn build_weighted_graph() {
    let mut graph: Vec<Vec<WeightedNode>>;
}
