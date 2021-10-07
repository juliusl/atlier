#define CIMGUI_DEFINE_ENUMS_AND_STRUCTS
#include <cimgui.h>
#include <cimnodes.h>
#include <cimgui_impl.h>
#include "cimgui_extras.h"
#include <stdio.h>
#define SDL_MAIN_HANDLED
#include <SDL.h>
#ifdef _MSC_VER
#include <windows.h>
#endif
#include <GL/gl.h>
#include <GL/glu.h>
#include <git2.h>

#ifdef IMGUI_HAS_IMSTR
#define igBegin igBegin_Str
#define igSliderFloat igSliderFloat_Str
#define igCheckbox igCheckbox_Str
#define igColorEdit3 igColorEdit3_Str
#define igButton igButton_Str
#endif

SDL_Window* window = NULL;

static void check_error(int error_code, const char* action)
{
	const git_error* error = git_error_last();
	if (!error_code)
		return;

	printf("Error %d %s - %s\n", error_code, action,
		(error && error->message) ? error->message : "???");

	exit(1);
}
enum {
	FORMAT_DEFAULT = 0,
	FORMAT_LONG = 1,
	FORMAT_SHORT = 2,
	FORMAT_PORCELAIN = 3,
};

static void show_branch(git_repository* repo, int format)
{
	int error = 0;
	const char* branch = NULL;
	git_reference* head = NULL;

	error = git_repository_head(&head, repo);

	if (error == GIT_EUNBORNBRANCH || error == GIT_ENOTFOUND)
		branch = NULL;
	else if (!error) {
		branch = git_reference_shorthand(head);
	}
	else
		check_error(error, "failed to get current branch");

	if (format == FORMAT_LONG)
		printf("# On branch %s\n",
			branch ? branch : "Not currently on any branch.");
	else
		printf("## %s\n", branch ? branch : "HEAD (no branch)");

	git_reference_free(head);
}

int main(int argc, char* argv[]) 
{
	git_repository* repo;
	char* repo_path; 
	int error;
	git_oid oid;

	git_libgit2_init();

	repo_path = "C:\\Users\\juliusl\\src\\atlier\\deps\\cimgui";
	error = git_repository_open(&repo, repo_path);
	check_error(error, "opening repository");

	show_branch(repo, FORMAT_SHORT);
	git_repository_free(repo);

	if (SDL_Init(SDL_INIT_VIDEO) < 0) {
		SDL_Log("failed to init: %s", SDL_GetError());
		return -1;
	}

	// Decide GL+GLSL versions
#if __APPLE__
	// GL 3.2 Core + GLSL 150
	const char* glsl_version = "#version 150";
	SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG); // Always required on Mac
	SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
	SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
	SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 2);
#else
	// GL 3.0 + GLSL 130
	const char* glsl_version = "#version 130";
	SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, 0);
	SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
	SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
	SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
#endif

	// and prepare OpenGL stuff
	SDL_SetHint(SDL_HINT_RENDER_DRIVER, "opengl");
	SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 24);
	SDL_GL_SetAttribute(SDL_GL_STENCIL_SIZE, 8);
	SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
	SDL_DisplayMode current;
	SDL_GetCurrentDisplayMode(0, &current);

	window = SDL_CreateWindow(
		"Atlier", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 1024, 768,
		SDL_WINDOW_SHOWN | SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE
	);
	if (window == NULL) {
		SDL_Log("Failed to create window: %s", SDL_GetError());
		return -1;
	}

	SDL_GLContext gl_context = SDL_GL_CreateContext(window);
	SDL_GL_SetSwapInterval(1);  // enable vsync

	// Initialize OpenGL loader for cimgui_sdl
	bool err = Do_gl3wInit() != 0;
	if (err)
	{
		SDL_Log("Failed to initialize OpenGL loader for cimgui_sdl!");
		return 1;
	}

	// check opengl version sdl uses
	SDL_Log("opengl version: %s", (char*)glGetString(GL_VERSION));

	// setup imgui
	igCreateContext(NULL);

	imnodes_CreateContext();

	//set docking
	ImGuiIO* ioptr = igGetIO();
	ioptr->ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;       // Enable Keyboard Controls
	//ioptr->ConfigFlags |= ImGuiConfigFlags_NavEnableGamepad;      // Enable Gamepad Controls
#ifdef IMGUI_HAS_DOCK
	ioptr->ConfigFlags |= ImGuiConfigFlags_DockingEnable;           // Enable Docking
	ioptr->ConfigFlags |= ImGuiConfigFlags_ViewportsEnable;         // Enable Multi-Viewport / Platform Windows
#endif

	ImGui_ImplSDL2_InitForOpenGL(window, gl_context);
	ImGui_ImplOpenGL3_Init(glsl_version);

	igStyleColorsDark(NULL);
	//ImFontAtlas_AddFontDefault(io.Fonts, NULL);


	bool showDemoWindow = true;
	bool showAnotherWindow = false;
	ImVec4 clearColor;
	clearColor.x = 0.45f;
	clearColor.y = 0.55f;
	clearColor.z = 0.60f;
	clearColor.w = 1.00f;

	bool quit = false;
	while (!quit)
	{
		SDL_Event e;

		// we need to call SDL_PollEvent to let window rendered, otherwise
		// no window will be shown
		while (SDL_PollEvent(&e) != 0)
		{
			ImGui_ImplSDL2_ProcessEvent(&e);
			if (e.type == SDL_QUIT)
				quit = true;
			if (e.type == SDL_WINDOWEVENT && e.window.event == SDL_WINDOWEVENT_CLOSE && e.window.windowID == SDL_GetWindowID(window))
				quit = true;
		}

		// start imgui frame
		ImGui_ImplOpenGL3_NewFrame();
		ImGui_ImplSDL2_NewFrame(window);
		igNewFrame();

		if (showDemoWindow)
			igShowDemoWindow(&showDemoWindow);

		// show a simple window that we created ourselves.
		{
			static float f = 0.0f;
			static int counter = 0;

			igBegin("Hello, world!", NULL, 0);
			igText("This is some useful text");
			igCheckbox("Demo window", &showDemoWindow);
			igCheckbox("Another window", &showAnotherWindow);

			igSliderFloat("Float", &f, 0.0f, 1.0f, "%.3f", 0);
			igColorEdit3("clear color", (float*)&clearColor, 0);

			ImVec2 buttonSize;
			buttonSize.x = 0;
			buttonSize.y = 0;
			if (igButton("Button", buttonSize))
				counter++;
			igSameLine(0.0f, -1.0f);
			igText("counter = %d", counter);

			igText("Application average %.3f ms/frame (%.1f FPS)", 1000.0f / igGetIO()->Framerate, igGetIO()->Framerate);
			igEnd();
		}

		if (showAnotherWindow)
		{
			igBegin("imgui Another Window", &showAnotherWindow, 0);
			igText("Hello from imgui");
			ImVec2 buttonSize;
			buttonSize.x = 0; buttonSize.y = 0;
			if (igButton("Close me", buttonSize))
			{
				showAnotherWindow = false;
			}
			igEnd();
		}


		{
			igBegin("simple node editor", NULL, 0);

			imnodes_BeginNodeEditor();
			imnodes_BeginNode(1);

			imnodes_BeginNodeTitleBar();
			igTextUnformatted("simple node :)", NULL);
			imnodes_EndNodeTitleBar();

			imnodes_BeginInputAttribute(2, ImNodesPinShape_Circle);
			igText("input");
			imnodes_EndInputAttribute();

			imnodes_BeginOutputAttribute(3, ImNodesPinShape_Circle);
			igIndent(40);
			igText("output");
			imnodes_EndOutputAttribute();

			imnodes_EndNode();

			imnodes_BeginNode(4);

			imnodes_BeginNodeTitleBar();
			igTextUnformatted("simple node :)", NULL);
			imnodes_EndNodeTitleBar();

			imnodes_BeginInputAttribute(5, ImNodesPinShape_Circle);
			igText("input");
			imnodes_EndInputAttribute();

			imnodes_BeginOutputAttribute(6, ImNodesPinShape_Circle);
			igIndent(40);
			igText("output");
			imnodes_EndOutputAttribute();

			imnodes_EndNode();
			imnodes_EndNodeEditor();

			igEnd();
		}

		// render
		igRender();
		SDL_GL_MakeCurrent(window, gl_context);
		glViewport(0, 0, (int)ioptr->DisplaySize.x, (int)ioptr->DisplaySize.y);
		glClearColor(clearColor.x, clearColor.y, clearColor.z, clearColor.w);
		glClear(GL_COLOR_BUFFER_BIT);
		ImGui_ImplOpenGL3_RenderDrawData(igGetDrawData());
#ifdef IMGUI_HAS_DOCK
		if (ioptr->ConfigFlags & ImGuiConfigFlags_ViewportsEnable)
		{
			SDL_Window* backup_current_window = SDL_GL_GetCurrentWindow();
			SDL_GLContext backup_current_context = SDL_GL_GetCurrentContext();
			igUpdatePlatformWindows();
			igRenderPlatformWindowsDefault(NULL, NULL);
			SDL_GL_MakeCurrent(backup_current_window, backup_current_context);
		}
#endif
		SDL_GL_SwapWindow(window);
	}

	// clean up
	ImGui_ImplOpenGL3_Shutdown();
	ImGui_ImplSDL2_Shutdown();
	igDestroyContext(NULL);
	imnodes_DestroyContext(NULL);

	SDL_GL_DeleteContext(gl_context);
	if (window != NULL)
	{
		SDL_DestroyWindow(window);
		window = NULL;
	}
	SDL_Quit();

	return 0;
}