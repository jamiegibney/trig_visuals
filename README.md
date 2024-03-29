# Trigonometric function visualisation

https://github.com/jamiegibney/trig_visuals/assets/123845103/920f95a5-1bc3-4ae4-a7cd-52850d61f3cc

A simple visualisation of common trigonometric functions on a [unit circle](https://en.wikipedia.org/wiki/Unit_circle).

Made using the [nannou](https://github.com/nannou-org/nannou) library for Rust. Inspired by [this video](https://youtu.be/Dsf6ADwJ66E?si=xC_gJOOfiLqyZQ35).

## Visualised functions

$$\mathrm{sin}\ θ = \frac{O}{H}$$
$$\mathrm{cos}\ θ = \frac{A}{H}$$
$$\mathrm{tan}\ θ = \frac{O}{A}$$
$$\mathrm{csc}\ θ = \frac{H}{O} = \frac{1}{\mathrm{sin}\ θ}$$
$$\mathrm{sec}\ θ = \frac{H}{A} = \frac{1}{\mathrm{cos}\ θ}$$
$$\mathrm{cot}\ θ = \frac{A}{O} = \frac{1}{\mathrm{tan}\ θ}$$

</br>

$$A = \mathrm{Adjacent\ side}\\ O = \mathrm{Opposite\ side}\\ H = \mathrm{Hypotenuse}\\ θ = \mathrm{Theta\ (current\ angle\ in\ radians)}$$

## Keymap
- `Space` → toggle motion
- `L` → toggle labels (attached to coloured lines)
- `V` → toggle right-hand side values
- `T` → toggle visual of theta $θ$
- `R` → reset theta $θ$
- `S` → reset motion rate
- `Up` → increase motion rate
- `Down` → decrease motion rate
