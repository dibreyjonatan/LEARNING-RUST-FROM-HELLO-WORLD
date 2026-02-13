
.PHONY : clean, build,assemble, run, all 
all :  assemble  
	
clean : 
	rm bin || :
	rm *.rlib ||:

build :
	rustc --crate-type=lib bib.rs
	rustc --crate-type=lib math.rs 

assemble : build 
	rustc code.rs --extern utils=*.rlib --crate-name bin 
# *.rlib selectionne tous les fichiers lib		      		      
run :  assemble 
	./bin 	      		      
		      		      
