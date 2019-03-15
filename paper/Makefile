all: sigbovik.pdf

%.pdf: %.tex
	pdflatex -synctex=1 -interaction=nonstopmode -shell-escape $<
	bibtex $*
	pdflatex -synctex=1 -interaction=nonstopmode -shell-escape $<
	pdflatex -synctex=1 -interaction=nonstopmode -shell-escape $<

clean:
	rm *.bbl *.dvi *.log *.pdf *.aux *.synctex.gz || true
