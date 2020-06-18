all:
	clang++ main.cpp rules.cpp driver.cpp -o app
	clang++ rules.cpp driver.cpp test.cpp -o test
clean:
	rm -f app test
test: all
	./test
