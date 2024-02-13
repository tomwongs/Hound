#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>
#include <unistd.h>
#include <time.h>
#include <ctype.h>

// File Creation: 11/02/2024 16:24

#define SUCCESS 0
#define ERR -1
#define YES 0
#define NO 1

int help();
int fDestroy();

char color[7][10]={
	"\033[0m",	// 0: Default
	"\033[91m",	// 1: Red
	"\033[32m",	// 2: Green (files)
	"\033[36m",	// 3: Cyan (folders)
	"\033[33m",	// 4: Highlight
	"\033[4m",	// 5: Underline
	"\033[1m"	// 6: Bold
};

char houndIcon[] =
"\033[1;91m"
"        :.^^~^^:....                                        \n"
"      .^777777777777!~^.                                    \n"
" ^:  .!77777777777777777!^.                                 \n"
" YJ.^77777777777777777777777!:                              \n"
" ?Y5J7777777777777777777!!!!!77~:                           \n"
" ~YY?!!7777?777777777777!!!!!!7777~.                        \n"
" ^777!777J?:^^~7777777~~7777??7!!777^                       \n"
" .J!!~~!7.     ..:7777~~!:..:~7!!77777~:                    \n"
"  ^~^?P^.       .!?!:..~.      ^777777!7:                   \n"
"    :JG^       ^7^.    .^.     ^777777~~~                   \n"
"    :!J.      !~          ..  ~777777~~~~~^^:.              \n"
"              ^^^!~^.       .!777!!!~^^^~~^!~^^~~:..        \n"
"                           .7777!~^:~~~~^~.:^~~~~~~~~^:     \n"
"                           :?777~~. :!!~^~:  .:^^~~~~~~^.   \n"
"                            .~!!!7!  :77!^^      ^~~~~~~~:  \n"
"                                .^!7^::^7!~       :~~~~~~^: \n"
"                                   ^?7.  ~7^       .~~~~~~: \n"
"                                   7!     .7~.     .~~~~~~^.\n"
"                                 :!!       .77:     :~~~~~. \n"
"                                ~~:         !!      ~~::~:  \n"
"                                :..         ?!      :~.     \n"
"                                           .7~              \n"
"                                           :^.              \n"
"                                            :               "
"\033[1;0m\n\n";

int main(int argCount, char *argValue[]) {

	int info[2] = {
		0,	// isChomp
		1	// angryValue
	};

	if (argCount<2) { help(); return SUCCESS; }

	for (int i=1; i<argCount; i++) {
		
		if (strcmp(argValue[i], "-h") == 0 || strcmp(argValue[i], "--help") == 0) {
			help();
			return SUCCESS;
		}

		if (strcmp(argValue[i], "-c") == 0 || strcmp(argValue[i], "--chomp") == 0) {
			info[0] = 1;
			continue;
		}

		fDestroy(argValue[i], info);
	}

	return SUCCESS;
}



char generateRndChar(char const *list) {
	int nb = rand() % 87 +1;
	return list[nb];
}


int isDirectory(char *targetPath) {
	struct stat targetStat;

	if (stat(targetPath, &targetStat) == 0) {
		if (S_ISDIR(targetStat.st_mode)) {
			return YES;

		} else {
			return NO;
		}

	} else {
		printf("Hound couldn't collect the information on the target.");
		return ERR;
	}
}

int fDelete(char *targetPath, char *info) {
	if (info[0] != 0) { return SUCCESS; }

	if (remove(targetPath) == SUCCESS) {
		printf("%s%s%s \t: Devoured!\n", color[2], targetPath, color[0]);
		return SUCCESS;
	}
	else {
		printf("%s%s \t: Couldn't Devour!%s\n", color[4], targetPath, color[0]);
		return ERR;
	}

	printf("Unknown error on deciding if Hound should devour or not the target.");
	return ERR;
}


int dDestroy(char *targetPath) {
	printf("Hound cannot yet take care of directories.\n");
	return SUCCESS;
}


int fDestroy(char *targetPath, char *info) {

	int isDir;
	long targetBytes;
	FILE *fTarget;

	// Check if the target exist.
	if (access(targetPath, F_OK) != SUCCESS) {
		printf("The target doesn't exist!");
		return ERR;
	}

	// Dir Manager.
	isDir = isDirectory(targetPath);
	if (isDir == ERR) { return ERR; }

	if (isDir == YES) {
		return dDestroy(targetPath);
	}

	// Get the file size.
	fTarget = fopen(targetPath, "rb+");
	fseek(fTarget, 0, SEEK_END);
	targetBytes = ftell(fTarget);
	fseek(fTarget, 0, SEEK_SET);

	srand(time(NULL));	// set the seed for the random func.
	unsigned char fContent = rand() % 2;
	fContent <<= rand() % 7;


	// Chomp
	if (fwrite(&fContent, sizeof(fContent), targetBytes, fTarget) >= 0) {
		printf("%s%s%s \t: %sChomped!%s\n", color[2], targetPath, color[0], color[1], color[0]);

	} else {
		printf("%s%s \t: Cannot Chomp!%s\n", color[4], targetPath, color[0]);
		info[0]=1;
	}


	fclose(fTarget);
	return fDelete(targetPath, info);
}

int help() {
	printf(houndIcon);
	printf("Hound is a software made to destroy data without any recovery chances.\n");
	printf("Usage : hound <switches..> [folder/file]\n\n");
	printf("%s<Switches>%s\n", color[6], color[0]);
	printf("-a (--angry):\t the file will get massacred [x] more time.\n");
	printf("-c (--chomp):\t doesn't erase the file at the end.\n\n");
	printf("%sExamples:%s 'hound -a=6c [file]'\n\n", color[6], color[0]);
	printf("%s! Disclamer !%s\nPlease, use hound only, on your %s%sOWN%s data!\n", color[6], color[0], color[5],color[6],color[0]);
	printf("The author of the program is not responsible for any actions made with this program.\n");
	return SUCCESS;
}
