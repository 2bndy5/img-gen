Colors
======

.. autoclass:: img_gen.ColorKind

    .. autoattribute:: SolidColor
        :annotation: : Callable[[Color], ColorKind]

        Pass an instance of `Color` to the parameter of this constructor.

    .. autoattribute:: LinearGradient
        :annotation: : Callable[[LinearGradient], ColorKind]

        Pass an instance of `LinearGradient` to the parameter of this constructor.

    .. autoattribute:: RadialGradient
        :annotation: : Callable[[RadialGradient], ColorKind]

        Pass an instance of `RadialGradient` to the parameter of this constructor.

    .. autoattribute:: ConicalGradient
        :annotation: : Callable[[ConicalGradient], ColorKind]

        Pass an instance of `ConicalGradient` to the parameter of this constructor.


Solid color
***********

.. autoclass:: img_gen.SolidColor
    :members:

Gradient colors
***************

.. toctree::
    :maxdepth: 1

    linear_gradient
    radial_gradient
    conical_gradient
    gradient_presets

.. autoclass:: img_gen.ColorGradient
    :members:

.. autoclass:: img_gen.Spread

    .. autoattribute:: Pad
    .. autoattribute:: Reflect
    .. autoattribute:: Repeat
