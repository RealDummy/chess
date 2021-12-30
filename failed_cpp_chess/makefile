
INC_DIR := include
SRC_DIR := source
OBJ_DIR := obj

TARGET := chess

COMPILER := clang++
STD := c++20
FLAGS := -I ./$(INC_DIR) --std=$(STD) -Wall -O2

SRCS := $(wildcard $(SRC_DIR)/*.cpp)
INCS := $(wildcard $(INC_DIR)/*.h)


#compile object files as needed
$(OBJ_DIR)/%.o : $(SRC_DIR)/%.cpp $(INCS)
	$(COMPILER) -c -o $@ $< $(FLAGS)

$(TARGET) : $(patsubst $(SRC_DIR)/%.cpp, $(OBJ_DIR)/%.o , $(SRCS))
	$(COMPILER) $^ $(FLAGS) -o $@

.PHONY : clean

clean : 
	rm $(wildcard $(OBJ_DIR)/*.o)
