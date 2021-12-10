all:
	pwd
	rustc ~/coding/Learning_Langs/Learning_Rust/src/main.rs
	g++ ~/coding/Learning_Langs/Learning_Cpp/src/main.cpp 
	./a.out
	clear
	./main
	clear
	rm a.out
	rm main

rust:
	cd ~/coding/Learning_Langs/Learning_Rust \
		cargo run && cargo clean

cpp:
	g++ ~/coding/Learning_Langs/Learning_Cpp/src/main.cpp
	clear
	./a.out

clean:
	rm a.out
	rm main
