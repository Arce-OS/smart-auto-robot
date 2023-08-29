#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <termios.h>

int main() {
    int fd; // 串口文件描述符

    // 打开串口设备
    // fd = open("/dev/ttyS0", O_RDWR | O_NOCTTY);
    fd = open("/dev/ttyUSB2", O_RDWR | O_NOCTTY);
    if (fd == -1) {
        perror("无法打开串口");
        exit(EXIT_FAILURE);
    }

    // 配置串口参数
    struct termios options;
    tcgetattr(fd, &options);
    cfsetispeed(&options, B115200); // 设置输入波特率
    cfsetospeed(&options, B115200); // 设置输出波特率
    options.c_cflag |= (CLOCAL | CREAD); // 启用接收器和本地模式
    options.c_cflag &= ~PARENB; // 禁用奇偶校验
    options.c_cflag &= ~CSTOPB; // 设置停止位为1个
    options.c_cflag &= ~CSIZE; // 清除数据位设置
    options.c_cflag |= CS8; // 设置数据位为8位
    tcsetattr(fd, TCSANOW, &options);

    // 向串口发送数据
    char data[] = "Hello, Serial!";
    write(fd, data, sizeof(data));

    // 从串口读取数据
    char buffer[255];
    int bytesRead = read(fd, buffer, sizeof(buffer));
    if (bytesRead > 0) {
        printf("接收到的数据：%s\n", buffer);
    }

    // 关闭串口
    close(fd);

    return 0;
}
