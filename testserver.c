#include <stdio.h>      // Standard input/output definitions
#include <stdlib.h>     // Standard library
#include <string.h>     // String function definitions
#include <unistd.h>     // UNIX standard function definitions
#include <sys/types.h>  // Data types
#include <sys/socket.h> // Socket definitions
#include <netinet/in.h> // Internet address family

#define PORT 12345  // The port number you want the server to listen on

int main() {
	//CREATE SOCKET
	int sockfd = socket(AF_INET, SOCK_STREAM, 0);
	if (sockfd < 0) {
	    perror("ERROR opening socket");
	    exit(1);
	}
	close(sockfd);
        return 0;


	//BIND SOCKET TO ADDRESS
	struct sockaddr_in serv_addr;
	memset(&serv_addr, 0, sizeof(serv_addr));
	serv_addr.sin_family = AF_INET;
	serv_addr.sin_addr.s_addr = INADDR_ANY;
	serv_addr.sin_port = htons(PORT);

	if (bind(sockfd, (struct sockaddr *) &serv_addr, sizeof(serv_addr)) < 0) {
	    perror("ERROR on binding");
	    close(sockfd);
	    exit(1);
	}
}


