.PHONY: resources
resources:
	find resources/ -iname "0*.svg" | parallel --bar ./src/gen_resources.py {}
	#
	cairosvg -o resources/about_flat2.svg resources/about_flat.svg && \
	mv resources/about_flat2.svg resources/about_flat.svg && \
	./src/gen_resources.py resources/about_flat.svg
