# stm32u083_pac
An attempt to build a Rust Peripheral Access Crate for the as yet unsupported STM32U083 microcontroller in hopes of using it to tie into the embedded_hal framework to create a HAL for this part.

I had issues with the SVD I copied from the STM32CubeMX package for this device.  Details about that are in the pac_construction_notes.txt file.  Next I hope to test a bit of this PAC on the ST Micro NUCLEO-U083RC board for this controllerr.
