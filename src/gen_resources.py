#!/usr/bin/env python3
# run for about_flat.svg:
# cairosvg -i resources/about_flat.svg -o /tmp/flat.svg && mv resources/about_flat.svg resources/about_flat_is.svg && mv /tmp/flat.svg resources/about_flat.svg && ./src/gen_resources.py resources/about_flat.svg

from xml.etree import ElementTree
import sys

(_, svgfile) = sys.argv

f = open(svgfile, "rb")
xml1 = ElementTree.ElementTree(file=f)
f.close()
pl = "\n".join([e.attrib["d"] for e in xml1.iter() if "d" in e.attrib])
f2 = open(svgfile+"p", "w+")
f2.write(pl)
f2.close()
