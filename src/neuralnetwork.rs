use rand::Rng;

pub struct NeuralNetwork {
    input_layer: Vec<Node>,
    hidden_layer: Vec<Node>,
    output_layer: Vec<Node>,
}

impl NeuralNetwork {
    pub fn new(inputs: usize, hiddens: usize, outputs: usize) -> NeuralNetwork {
        // create layers
        let mut input_layer: Vec<Node> = (0..inputs).map(|_| Node::new()).collect();
        let mut hidden_layer: Vec<Node> = (0..hiddens).map(|_| Node::new()).collect();
        let mut output_layer: Vec<Node> = (0..outputs).map(|_| Node::new()).collect();

        // input layers only need one edge for initial input, inital input -> input layer
        for node in input_layer.iter_mut() {
            node.edges.push(Edge::new(0));
        }

        // setup edges between rest of layers, input -> hidden and hidden -> output
        init_edges(&input_layer, &mut hidden_layer);
        init_edges(&hidden_layer, &mut output_layer);

        // return all the created data
        NeuralNetwork {
            input_layer,
            hidden_layer,
            output_layer,
        }
    }
    fn process(&mut self) {
        for i in 0..self.hidden_layer.len() {
            let mut sum = 0.;
            for j in 0..self.hidden_layer[i].edges.len() {
                let input = self.hidden_layer[i].edges[j].input;
                let weight = self.hidden_layer[i].edges[j].weight;
                sum += input * weight;
            }
            self.hidden_layer[i].output = sigmoid(sum);
        }
    }
    pub fn set_input(&mut self, index: usize, input: f32) {
        self.input_layer[index].edges[0].input = input;
    }
    pub fn get_output(&self, index: usize) -> f32 {
        self.output_layer[index].output
    }
}

#[derive(Clone)]
struct Edge {
    input: f32,
    index: usize,
    weight: f32,
}

impl Edge {
    fn new(index: usize) -> Edge {
        let mut rng = rand::thread_rng();
        let input = 0.;
        let weight = rng.gen_range(0., 1.);
        Edge {
            input,
            index,
            weight,
        }
    }
    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        self.weight = (self.weight + rng.gen_range(-0.1, 0.1)).max(0.).min(1.);
    }
}

#[derive(Clone)]
pub struct Node {
    edges: Vec<Edge>,
    output: f32,
}

impl Node {
    fn new() -> Node {
        let edges: Vec<Edge> = vec![];
        let output: f32 = 0.;
        Node {
            edges,
            output,
        }
    }
}

fn init_edges(previous_layer: &[Node], layer: &mut Vec<Node>) {
    let edges = previous_layer.len();
    for node in layer.iter_mut() {
        for i in 0..edges {
            node.edges.push(Edge::new(i));
        }
    }
}

fn sigmoid(x: f32) -> f32 {
    1. / (1. + std::f32::consts::E.powf(-x))
}
