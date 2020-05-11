use rand::Rng;

#[derive(Debug, Clone)]
pub struct NeuralNetwork {
    layers: Vec<Vec<Node>>,
}

impl NeuralNetwork {
    pub fn new(inputs: usize, hiddens: usize, outputs: usize) -> NeuralNetwork {
        let mut layers: Vec<Vec<Node>> = vec![];

        layers.push(create_layer(inputs, hiddens));
        layers.push(create_layer(hiddens, outputs));
        layers.push(create_layer(outputs, 0));

        println!("{:?}", layers);

        NeuralNetwork {
            layers,
        }
    }
    pub fn get(&self, layer_index: usize, node_index: usize) -> f32 {
        let layer = &self.layers[layer_index];
        layer[node_index].data
    }
    pub fn set(&mut self, node_index: usize, input: f32) {
        let layer = &mut self.layers[0];
        layer[node_index].data = input;
    }
    pub fn process(&mut self) {
        // hidden takes input from input, output takes input from hidden
        // hence why it starts from 1 then onward
        for layer in 1..self.layers.len() {
            let mut data: Vec<f32> = vec![];
            for node in 0..self.layers[layer].len() {
                let mut sum = 0.;
                let previous_layer = &self.layers[layer - 1];
                for previous_node in 0..previous_layer.len() {
                    sum += self.get_weighted(node, layer - 1, previous_node);
                }
                data.push(sigmoid(sum));
            }
            for (i, data) in data.iter().enumerate() {
                let current_layer = &mut self.layers[layer];
                current_layer[i].data = *data;
            }
        }
    }
    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        for layer in 1..self.layers.len() {
            for node in self.layers[layer].iter_mut() {
                for i in 0..node.weights.len() {
                    if rng.gen_range(0., 1.) > 0.5 {
                        node.weights[i] = (node.weights[i] + rng.gen_range(0., 1.)).max(0.).min(1.);
                    }
                }
            }
        }
    }
    fn get_weighted(&self, to_node_index: usize, layer_index: usize, from_node_index: usize) -> f32 {
        let layer = &self.layers[layer_index];
        let data = layer[from_node_index].data;
        let weight = layer[from_node_index].weights[to_node_index];
        data * weight
    }
}

#[derive(Debug, Clone)]
struct Node {
    data: f32,
    weights: Vec<f32>,
}

impl Node {
    fn new(weights: Vec<f32>) -> Node {
        Node {
            data: 0.,
            weights,
        }
    }
}

fn create_layer(amount: usize, next_layer_amount: usize) -> Vec<Node> {
    let mut rng = rand::thread_rng();
    (0..amount).map(|_| Node::new((0..next_layer_amount).map(|_| rng.gen_range(0., 1.)).collect())).collect()
}

fn sigmoid(x: f32) -> f32 {
    1. / (1. + std::f32::consts::E.powf(-x))
}
