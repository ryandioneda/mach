CXX := g++
CXXFLAGS := -std=c++20 -Wall -Wextra -mwindows -Iinclude
LDFLAGS := -lole32 -lshell32 -luuid

SRC_DIR := src
BUILD_DIR := build

SRCS := \
				$(SRC_DIR)/main.cpp 

OBJS := $(SRCS:$(SRC_DIR)/%.cpp=$(BUILD_DIR)/%.o)
TARGET := $(BUILD_DIR)/mach.exe

.PHONY clean run stop all

all: stop $(TARGET)

$(TARGET): $(OBJS)
	@powershell -Command "New-Item -ItemType Directory -Path '$(BUILD_DIR)' -Force" >nul 2>&1
	$(CXX) $(CXXFLAGS) -o $@ $^ $(LDFLAGS)

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cpp
	@powershell -Command "New-Item -ItemType Directory -Path '$(dir $@)' -Force" >nul 2>&1
	$(CXX) $(CXXFLAGS) -c $< -o $@

stop:
	@powershell -Command "if (Get-Process -Name macro_key_tool -ErrorAction SilentlyContinue) { Stop-Process -Name macro_key_tool -Force; Write-Host 'Stopped running instance.' }"

run: all
	@cmd /C start "" $(TARGET)

clean:
	@powershell -Command "if (Get-Process -Name macro_key_tool -ErrorAction SilentlyContinue) { Stop-Process -Name macro_key_tool -Force }"
	@if exist build rmdir /S /Q build



