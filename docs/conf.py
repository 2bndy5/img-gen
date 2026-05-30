# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html
from pathlib import Path
import asyncio
from img_gen import (
    Presets,
    Layout,
    Size,
    Offset,
    Layer,
    ColorKind,
    Generator,
    Background,
    Ellipse,
)


# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "img-gen"
copyright = "2024, Brendan Doherty"
author = "Brendan Doherty"
REPO_URL = "https://github.com/2bndy5/img-gen"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    "sphinx_immaterial",
    "sphinx.ext.autodoc",
    "sphinx.ext.intersphinx",
    "sphinx_jinja",
]

intersphinx_mapping = {
    "python": ("https://docs.python.org/3/", None),
}

templates_path = ["_templates"]
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]

default_role = "py:obj"
# add_module_names = False
# autodoc_class_signature = "separated"

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "sphinx_immaterial"
html_title = "img-gen"
html_static_path = ["_static"]
html_favicon = "_static/favicon.ico"
html_logo = "_static/logo.png"
html_theme_options = {
    "icon": {
        "repo": "fontawesome/brands/github",
        "edit": "material/file-edit-outline",
        # "logo": "material/image-edit",
    },
    "repo_url": REPO_URL,
    "repo_name": "img-gen",
    "site_url": "https://2bndy5.github.io/img-gen/",
    "edit_uri": "blob/main/docs",
    "features": [
        "navigation.expand",
        # "navigation.tabs",
        # "toc.integrate",
        "navigation.sections",
        # "navigation.instant",
        "navigation.top",
        # "navigation.tracking",
        "search.share",
        "toc.follow",
        "toc.sticky",
        "content.tabs.link",
        "announce.dismiss",
    ],
    "palette": [
        {
            "media": "(prefers-color-scheme)",
            "toggle": {
                "icon": "material/brightness-auto",
                "name": "Switch to light mode",
            },
        },
        {
            "media": "(prefers-color-scheme: light)",
            "scheme": "default",
            "primary": "teal",
            "accent": "purple",
            "toggle": {
                "icon": "material/lightbulb-outline",
                "name": "Switch to dark mode",
            },
        },
        {
            "media": "(prefers-color-scheme: dark)",
            "scheme": "slate",
            "primary": "teal",
            "accent": "purple",
            "toggle": {
                "icon": "material/lightbulb",
                "name": "Switch to light mode",
            },
        },
    ],
    "social": [
        {
            "icon": "fontawesome/brands/github",
            "link": REPO_URL,
            "name": "Source on github.com",
        },
        {
            "icon": "fontawesome/brands/python",
            "link": "https://pypi.org/project/img-gen/",
        },
        {
            "icon": "fontawesome/brands/rust",
            "link": "https://crates.io/crates/img-gen",
        },
    ],
}


async def generate_preset_examples() -> dict[int, str]:
    """Generate example images for each preset

    Return a mapping of preset int value to preset name.
    """
    presets = {}
    preset_examples = Path(__file__).parent / "preset_examples"
    preset_examples.mkdir(parents=True, exist_ok=True)
    generator = Generator()
    for p in dir(Presets):
        # cycle through all the presets and generate example images for each
        if not p.startswith("_"):
            preset = getattr(Presets, p)
            presets[int(preset)] = p
            preset_example_img = preset_examples / f"{p}.png"
            if not preset_example_img.exists():
                print("Generating", str(preset_example_img))
                layout = Layout(
                    size=Size(256 * 3, 256),
                    layers=[
                        Layer(
                            size=Size(256, 256),
                            ellipse=Ellipse(
                                color=ColorKind.radial_gradient(
                                    preset=preset,
                                    center=Offset(128, 128),
                                    radius=128,
                                )
                            ),
                        ),
                        Layer(
                            size=Size(256, 256),
                            offset=Offset(x=256),
                            background=Background(
                                color=ColorKind.linear_gradient(
                                    preset=preset,
                                    start=Offset(y=128),
                                    end=Offset(256, 128),
                                )
                            ),
                        ),
                        Layer(
                            size=Size(256, 256),
                            offset=Offset(x=512),
                            ellipse=Ellipse(
                                color=ColorKind.conical_gradient(
                                    preset=preset,
                                    center=Offset(128, 128),
                                )
                            ),
                        ),
                    ],
                )
                img = await generator.render(layout)
                img.save(str(preset_example_img))
    return presets


jinja_contexts = {
    "presets": {"presets": sorted(asyncio.run(generate_preset_examples()).items())}
}
