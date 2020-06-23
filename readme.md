Exercises from the "Deep Learning on ARM processors from the ground up" course:

https://www.udemy.com/course/deep-learning-from-ground-uptm-on-arm-processors/

ported to Rust.

So far got to lecture 14, the following functions are available from the helpers/nn module:

* multiple input, single output
* single input, multiple output
* multiple input, multiple output
* multiple input, multiple output with hidden layer

with the following private functions they use:

* weighted sum
* elementwise multiply
* matrix vector multplication

The target is STM32L0xx MCU, in my case Nucleo-32 STM32L031K6 dev board. To experiment with low power consumption the code uses MSI oscillator as main clock source, and LSE 32 kHz oscillator as Low Power UART clock source. 

It's my work in progress, so things are just commented out for now, as I go ahead with the course :)

__updates__:

* most arrays used as function arguments are now array slices, without a specified length, except for 2D arrays

__TO DO__:

* find out how to use a 2D array slice -> use 1D array instead, calculating indices, i.e.:

[[1,2,3], [4,5,6], [7,8.9]] becomes [1,2,3,4,5,6,7,8,9] 

* move the input, weights, indices etc. into model.rs, to have the following structure: app, NN lib, model
