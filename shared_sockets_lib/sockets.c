#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <poll.h>
#include <sys/epoll.h>
#include <dlfcn.h>

int socket(int domain, int type, int protocol) {
    // Original 'socket' function pointer
    static int (*original_socket)(int, int, int) = NULL;

    if (!original_socket) {
        // Dynamically load the original 'socket' function
        // RLTD_NEXT - skip the current function preventing recursive dlsym calls
        original_socket = dlsym(RTLD_NEXT, "socket");
    }
    // Check if it's an IPv4 TCP socket
    if (domain == AF_INET && type == SOCK_STREAM && (protocol == IPPROTO_TCP || protocol == 0)) {
        // Your custom implementation here
        //send socket message to "/tmp/wtfistcp_unix_socket"
    } else {
        // For other types of sockets, call the original socket function
        return original_socket(domain, type, protocol);
    }
}

int bind(int sockfd, const struct sockaddr *addr, socklen_t addrlen) {
    // Original 'bind' function pointer
    static int (*original_bind)(int, const struct sockaddr *, socklen_t) = NULL;

    if (!original_bind) {
        // Dynamically load the original 'bind' function
        original_bind = dlsym(RTLD_NEXT, "bind");
    }

    // Add your logic to check if sockfd corresponds to an IPv4 TCP socket
    // If yes, your custom implementation here
    // If no, call the original bind function
    return original_bind(sockfd, addr, addrlen);
}

int listen(int sockfd, int backlog) {
    // Original 'listen' function pointer
    static int (*original_listen)(int, int) = NULL;

    if (!original_listen) {
        // Dynamically load the original 'listen' function
        original_listen = dlsym(RTLD_NEXT, "listen");
    }

    // Add your logic to check if sockfd corresponds to an IPv4 TCP socket
    // If yes, your custom implementation here
    // If no, call the original listen function
    return original_listen(sockfd, backlog);
}

// ... Similarly for other functions like connect, accept, send, recv, etc. ...


int accept(int sockfd, struct sockaddr *addr, socklen_t *addrlen);
int connect(int sockfd, const struct sockaddr *addr, socklen_t addrlen);
int close(int sockfd);
int getsockopt(int sockfd, int level, int optname, void *optval, socklen_t *optlen);
int setsockopt(int sockfd, int level, int optname, const void *optval, socklen_t optlen);

// Data transmission
ssize_t send(int sockfd, const void *buf, size_t len, int flags);
ssize_t recv(int sockfd, void *buf, size_t len, int flags);
ssize_t sendto(int sockfd, const void *buf, size_t len, int flags, const struct sockaddr *dest_addr, socklen_t addrlen);
ssize_t recvfrom(int sockfd, void *buf, size_t len, int flags, struct sockaddr *src_addr, socklen_t *addrlen);
ssize_t sendmsg(int sockfd, const struct msghdr *msg, int flags);
ssize_t recvmsg(int sockfd, struct msghdr *msg, int flags);

// Multiplexing and asynchronous I/O
int select(int nfds, fd_set *readfds, fd_set *writefds, fd_set *exceptfds, struct timeval *timeout);
int poll(struct pollfd *fds, nfds_t nfds, int timeout);
int epoll_create(int size);
int epoll_ctl(int epfd, int op, int fd, struct epoll_event *event);
int epoll_wait(int epfd, struct epoll_event *events, int maxevents, int timeout);
