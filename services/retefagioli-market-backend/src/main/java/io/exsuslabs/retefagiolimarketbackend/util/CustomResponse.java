package io.exsuslabs.retefagiolimarketbackend.util;

import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;

import java.util.HashMap;
import java.util.Map;

public class CustomResponse {
    private Map<Object, Object> result = new HashMap<>();
    private HttpStatus status;

    public static CustomResponse create() {
        return new CustomResponse();
    }

    public CustomResponse message(String message) {
        result.put("message", message);
        return this;
    }

    public CustomResponse error_message(String error_message) {
        result.put("error_message", error_message);
        return this;
    }

    public CustomResponse status(HttpStatus status) {
        result.put("code", status.value());
        this.status = status;
        return this;
    }

    public CustomResponse addModel(Object model) {
        result.put("user", model);
        return this;
    }

    public ResponseEntity<Map<Object, Object>> build() {
        return new ResponseEntity<>(result, status);
    }

}
