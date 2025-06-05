#include <windows.h>

int WINAPI wWinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance,
                    LPWSTR lpCmdLine, int nShowCmd) {
  OutputDebugStringW(L"[MAIN] Hello, World!");
  return 0;
}
