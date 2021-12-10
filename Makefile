all:
	pwd
	rustc ~/coding/Learning_Langs/Learning_Rust/src/main.rs
	g++ ~/coding/Learning_Langs/Learning_Cpp/src/main.cpp
	./a.out
	./main
	rm a.out
	rm main

rust:
	rustc ~/coding/Learning_Langs/Learning_Rust/src/main.rs
	./main

cpp:
	g++ ~/coding/Learning_Langs/Learning_Cpp/src/main.cpp
	./a.out

clean:
	rm ./a.out
	rm ./main
