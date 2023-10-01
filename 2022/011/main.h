
#define TEAM_SIZE 4
#define DBG_PASS 0
#if (DBG_PASS == 1)
#define PASS_PRINTF(...) printf(__VA_ARGS__)
#else
#define PASS_PRINTF(...) (void)(__VA_ARGS__)
#endif
