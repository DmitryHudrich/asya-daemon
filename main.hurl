# REQUEST FOR TESTING STUFF

GET {{host}}/fetch
[QueryStringParams]
issys_name: 1
issys_kernelv: 1
issys_hostname: 1
issys_osv: 1
issys_osvlong: 1
issys_uptime: 1
issys_distroid: 1

GET {{host}}/fetch
[QueryStringParams]
ihram_totalmem: 1
ihram_usedmem: 1
ihram_freemem: 1
ihram_availmem: 1
ihram_totalswp: 1
ihram_freeswp: 1
ihram_usedswp: 1

GET {{host}}/fetch
[QueryStringParams]
ihcpu_name: 1
ihcpu_cores: 1

GET {{host}}/fetch
[QueryStringParams]
ihwtr_weather: 1
