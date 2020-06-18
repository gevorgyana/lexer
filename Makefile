all:
	clang++ main.cpp rules.cpp driver.cpp -o app
clean:
	rm -f app
