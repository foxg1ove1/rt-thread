#ifndef RTT_RUST_WRAPPER_H
#define RTT_RUST_WRAPPER_H

#include <rtthread.h>
#include <rtdevice.h>

// 基础类型重定义
typedef rt_thread_t rt_thread_handle_t;
typedef rt_sem_t rt_sem_handle_t;
typedef rt_mutex_t rt_mutex_handle_t;
typedef rt_mq_t rt_mq_handle_t;

// 线程相关函数
rt_thread_handle_t rt_thread_create(const char *name,
                                   void (*entry)(void *parameter),
                                   void *parameter,
                                   rt_uint32_t stack_size,
                                   rt_uint8_t priority,
                                   rt_uint32_t tick);
rt_err_t rt_thread_startup(rt_thread_handle_t thread);
rt_err_t rt_thread_delete(rt_thread_handle_t thread);
rt_err_t rt_thread_suspend(rt_thread_handle_t thread);
rt_err_t rt_thread_resume(rt_thread_handle_t thread);
// void rt_thread_delay(rt_tick_t tick);
// void rt_thread_mdelay(rt_int32_t ms);
rt_thread_handle_t rt_thread_self(void);
void rt_schedule(void);

// 信号量相关函数
rt_sem_handle_t rt_sem_create(const char *name, rt_uint32_t value, rt_uint8_t flag);
rt_err_t rt_sem_delete(rt_sem_handle_t sem);
rt_err_t rt_sem_take(rt_sem_handle_t sem, rt_int32_t time);
rt_err_t rt_sem_release(rt_sem_handle_t sem);

// 互斥量相关函数
rt_mutex_handle_t rt_mutex_create(const char *name, rt_uint8_t flag);
rt_err_t rt_mutex_delete(rt_mutex_handle_t mutex);
rt_err_t rt_mutex_take(rt_mutex_handle_t mutex, rt_int32_t time);
rt_err_t rt_mutex_release(rt_mutex_handle_t mutex);

// 消息队列相关函数
rt_mq_handle_t rt_mq_create(const char *name, rt_size_t msg_size, rt_size_t max_msgs, rt_uint8_t flag);
rt_err_t rt_mq_delete(rt_mq_handle_t mq);
rt_err_t rt_mq_send(rt_mq_handle_t mq, const void *buffer, rt_size_t size);
rt_err_t rt_mq_recv(rt_mq_handle_t mq, void *buffer, rt_size_t size, rt_int32_t timeout);

// 内存管理函数
void* rt_malloc(rt_size_t size);
void rt_free(void *ptr);
void* rt_realloc(void *ptr, rt_size_t newsize);
void* rt_calloc(rt_size_t count, rt_size_t size);

// 内存信息函数
// void rt_memory_info(rt_uint32_t *total, rt_uint32_t *used, rt_uint32_t *max_used);

// 打印函数
int rt_kprintf(const char *fmt, ...);

// MSH命令注册
int msh_cmd_register(const char *cmd, int (*cmd_func)(int argc, char **argv), const char *desc);

// 常量定义
#define RT_EOK                          0
#define RT_ERROR                        1
#define RT_ETIMEOUT                     2
#define RT_EFULL                        3
#define RT_EEMPTY                       4
#define RT_ENOMEM                       5
#define RT_ENOSYS                       6
#define RT_EBUSY                        7
#define RT_EIO                          8
#define RT_EINTR                        9
#define RT_EINVAL                       10

#define RT_WAITING_FOREVER              -1
#define RT_IPC_FLAG_FIFO                0x00
#define RT_IPC_FLAG_PRIO                0x01

#endif /* RTT_RUST_WRAPPER_H */