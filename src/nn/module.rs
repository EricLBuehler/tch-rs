use crate::data::Iter2;
use crate::tensor::Tensor;
use crate::Device;

pub trait Module {
    fn forward(&self, xs: &Tensor) -> Tensor;
}

pub trait ModuleT {
    fn forward_t(&self, xs: &Tensor, train: bool) -> Tensor;

    fn batch_accuracy_for_logits(
        &self,
        xs: &Tensor,
        ys: &Tensor,
        d: Device,
        batch_size: i64,
    ) -> f64 {
        let _no_grad = crate::NoGradGuard::new();
        let mut sum_accuracy = 0f64;
        let mut sample_count = 0f64;
        for (xs, ys) in Iter2::new(xs, ys, batch_size).return_smaller_last_batch() {
            let acc = self
                .forward_t(&xs.to_device(d), false)
                .accuracy_for_logits(&ys.to_device(d));
            let size = xs.size()[0] as f64;
            sum_accuracy += f64::from(&acc) * size;
            sample_count += size;
        }
        sum_accuracy / sample_count
    }
}

impl<T> ModuleT for T
where
    T: Module,
{
    fn forward_t(&self, xs: &Tensor, _train: bool) -> Tensor {
        self.forward(&xs)
    }
}

impl Tensor {
    pub fn apply<M: Module>(&self, m: &M) -> Tensor {
        m.forward(&self)
    }
    pub fn apply_t<M: ModuleT>(&self, m: &M, train: bool) -> Tensor {
        m.forward_t(&self, train)
    }
}