Exercises from the "Deep Learning on ARM processors from the ground up" course:

https://www.udemy.com/course/deep-learning-from-ground-uptm-on-arm-processors/

ported to Rust.

So far got to lecture 14, the following functions are available from the helpers/nn module:

* multiple input, single output
* single input, multiple output
* multiple input, multiple output

with the following private functions they use:

* weighted sum
* elementwise multiply
* matrix vector multplication

The target is STM32L0xx MCU, in my case Nucleo-32 STM32L031K6 dev board. To experiment with low power consumption the code uses MSI oscillator as main clock source, and LSE 32 kHz oscillator as Low Power UART source. 

It's my work in progress, so things are just commented out for now, as I go ahead with the course :)