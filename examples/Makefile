TARGET = example
LIBS = -lm -L../target/release -lc_bindings
CC = g++
CFLAGS = -I../

.PHONY: default all clean

default: $(TARGET)
all: default

# Change this line to look for .cpp files instead of .c files
OBJECTS = $(patsubst %.cpp, %.o, $(wildcard *.cpp))

# Fix the compilation rule syntax and make it use CFLAGS
%.o: %.cpp
	$(CC) $(CFLAGS) -c $< -o $@

.PRECIOUS: $(TARGET) $(OBJECTS)

$(TARGET): $(OBJECTS)
	$(CC) $(OBJECTS) -Wall $(LIBS) -o $@

clean:
	-rm -f *.o
	-rm -f $(TARGET)