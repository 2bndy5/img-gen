Gradient Presets
================

.. autoclass:: img_gen.Presets

    .. jinja:: presets

        {% for index, value in presets %}
        .. autoattribute:: {{ value }}
            :annotation: = {{ index }}

            .. image:: ../../../preset_examples/{{value}}.png
        {% endfor %}
