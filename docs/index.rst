``img-gen`` documentation
===================================

This python package (``img_gen``) generates images.
It is written in rust to encapsulate SVG support without needing
external dependencies (like cairo) installed.

.. toctree::
    :hidden:

    API/layout
    API/generator

.. toctree::
    :hidden:

    changelogs/index

.. spell-checker: disable-next-line
.. currentmodule:: img_gen

This library has 2 parts:

1. The :py:class:`Layout` specification (``img-gen-spec``) which defines the data structures for generating images.
2. The :py:class:`Generator` (``img-gen-renderer``) which takes the specification and renders an image.

Most of this documentation focuses on the specification because the rendering API is rather straight forward.

.. code-block:: python

    import asyncio
    from img_gen import Generator, Layout, Debug

    async def main():
        # Define the layout of the image
        layout = Layout(
            # nothing fancy, just an empty layout with debug outlines drawn
            debug=Debug(enable=True),
        )

        # Create a generator
        generator = Generator()
        # Generate the image
        image = await generator.render(layout)

        # Save the image to a file
        image.save(f"{image.sha256}.png")
