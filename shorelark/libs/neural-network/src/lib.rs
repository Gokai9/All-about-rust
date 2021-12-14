use rand::{thread_rng, Rng};

pub struct Network {
    layers: Vec<Layer>,
}
pub struct LayerTopology {
    neurons: usize,
}

impl Network {
    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        //self.layers.iter().fold(inputs, |inputs, layer| layer.propagate(inputs))
        for layer in &self.layers {
            inputs = layer.propagate(inputs)
        }
        inputs
    }
    pub fn random(layers: &[LayerTopology]) -> Self {
        // let mut built_lay = Vec::new();

        // for layer in layers.windows(2) {
        //     let input_neurons = layer[0].neurons;
        //     let output_neurons = layer[1].neurons;

        //     built_lay.push(Layer::random(input_neurons, output_neurons))
        // }
        let built_lay = layers
            .windows(2)
            .map(|layer| Layer::random(layer[0].neurons, layer[1].neurons))
            .collect();
        Self { layers: built_lay }
    }
}
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // self.neurons
        //     .iter()
        //     .map(|neuron| neuron.propagate(&inputs))
        //     .collect()
        let mut outputs = Vec::new();
        for neuron in &self.neurons {
            let output = neuron.propagate(&inputs);
            outputs.push(output);
        }
        outputs //jumlah outputs tergantung jumlah neuron
    }
    fn random(input: usize, output: usize) -> Self {
        // let mut neurons = Vec::new();

        // for _ in 0..output {
        //     neurons.push(Neuron::random(input))
        // }
        let neurons = (0..output).map(|_| Neuron::random(input)).collect();

        Self { neurons }
    }
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}
impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        let mut output = 0.0;
        assert_eq!(inputs.len(), self.weights.len());
        // for (&input, &weight) in inputs.iter().zip(&self.weights) {
        //     output = input * weight
        // }
        for i in 0..inputs.len() {
            output += inputs[i] * self.weights[i] //inputs and weight len has to be same
        }
        // output = inputs
        //     .iter()
        //     .zip(&self.weights)
        //     .map(|(input, weight)| input * weight)
        //     .sum::<f32>();
        output += self.bias;
        output.max(0.0)
    }
    fn random(input: usize) -> Self {
        let mut rng = thread_rng();
        let bias = rng.gen_range(-1.0..=1.0);
        let weights = (0..input).map(|_| rng.gen_range(-1.0..-1.0)).collect();
        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
