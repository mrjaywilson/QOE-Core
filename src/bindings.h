#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct SimConfig {
  uint32_t abr_type;
  uint32_t abr_window_size;
  float buffer_size_max;
  float segment_duration;
  float stall_threshold;
};

extern "C" {

void simulate_session();

float simulate_and_get_score();

float simulate_with_config_and_get_score(SimConfig config);

float simulate_with_config(SimConfig config);

char *simulate_and_get_json(SimConfig config);

void free_simulation_string(char *ptr);

}  // extern "C"
