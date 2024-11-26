local config = {
    net = {
        http_port = 3001,
        grpc_port = 50051
    },
    logging = {
        place = false,
        level = "info",
        folder = "logs",
        filescount = 5,
        stdout = true
    },
    telegram = {
        accepted_users = {
            "ohhh_i_l1ke_u"
        },
    },
    ai = {
        recognize_method = "AltaS",
        alta_s_path = "~/asya-model-server/",
        alta_s_addr = "http://127.0.0.1:5000/recognition",
    }
}

return config

