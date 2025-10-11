#!/bin/bash

rm -f *.log *.aux *.out *.toc *.lof *.lot *.fls *.fdb_latexmk *.bbl *.blg

# First compilation - generates .aux files
pdflatex -interaction=nonstopmode main.tex

# Second compilation - builds TOC and references
pdflatex -interaction=nonstopmode main.tex

# Third compilation - finalizes everything
pdflatex -interaction=nonstopmode main.tex

# Or use latexmk (recommended - does all passes automatically)
latexmk -pdf main.tex

# make this out put file exe
