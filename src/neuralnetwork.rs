use rand::Rng;
use ndarray::arr2;

pub struct NeuralNetwork<'a> {
    input: Vec<Node<'a>>,
    hidden: Vec<Node<'a>>,
    output: Vec<Node<'a>>,
}

impl<'a> NeuralNetwork<'a> {
    pub fn new(inputs: usize, hiddens: usize, outputs: usize) -> NeuralNetwork<'a> {
        let input: Vec<Node> = (0..inputs).map(|_| Node::new(inputs, inputs * hiddens)).collect();
        let hidden: Vec<Node> = (0..hiddens).map(|_| Node::new(inputs * hiddens, hiddens * outputs)).collect();
        let output: Vec<Node> = (0..outputs).map(|_| Node::new(hiddens * outputs, 0)).collect();
        NeuralNetwork {
            input,
            hidden,
            output,
        }
    }
}

#[derive(Clone)]
struct Edge<'a> {
    index: usize,
    from: &'a Node<'a>,
    weight: f64,
}

impl<'a> Edge<'a> {
    fn get_input(&self) -> f64 {
        self.from.input[self.index] * self.weight
    }
}

#[derive(Clone)]
struct Node<'a> {
    input: Vec<f64>,
    edges: &'a Vec<Edge<'a>>,
    output: f64,
}

impl<'a> Node<'a> {
    fn new(inputs: usize, links: usize) -> Node<'a> {
        let mut rng = rand::thread_rng();
        let input: Vec<f64> = (0..inputs).map(|_| 0.).collect();
        // let weights: Vec<f64> = (0..links).map(|_| rng.gen_range(0., 1.)).collect();
        let output: f64 = 0.;
        Node {
            input,
            edges,
            output,
        }
    }
    fn sigmoid(x: f64) -> f64 {
        1. / (1. + std::f64::consts::E.powf(-x))
    }
}
