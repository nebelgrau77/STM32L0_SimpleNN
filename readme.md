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

* all arrays used as function arguments are now array slices without a specified lenght
* in order to make it work in all cases, the weights matrices were flattened to 1D arrays
* all NN-model related indices, dimensions and weights were moved into model.rs: this should make the maintenance easier, and possibly allow to generate the model.rs file from a script doing the model training

__TO DO__:

* keep adding functions :)

__PROBLEMS__:

* there seems to be an issue with the LSE setup: it seems like the LSEON bit is not being set after power-up, at least most of the time. Reasons currently unknown.