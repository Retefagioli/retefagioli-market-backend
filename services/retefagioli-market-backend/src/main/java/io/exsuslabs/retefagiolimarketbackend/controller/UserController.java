package io.exsuslabs.retefagiolimarketbackend.controller;

import io.exsuslabs.retefagiolimarketbackend.request.UserFullInfoRequest;
import io.exsuslabs.retefagiolimarketbackend.service.JWTService;
import io.exsuslabs.retefagiolimarketbackend.service.UserService;
import io.exsuslabs.retefagiolimarketbackend.util.CustomResponse;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.validation.BindingResult;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.*;
import io.exsuslabs.retefagiolimarketbackend.model.UserModel;

import java.util.Objects;
import java.util.Optional;
import java.util.UUID;

@RestController
@RequestMapping("/user")
public class UserController {

    @Autowired
    UserService _userService;

    @GetMapping("/{id}")
    public ResponseEntity<?> getUser(@PathVariable UUID id) {
        Optional<UserModel> user;

        user = _userService.getUser(id);

        if (user.isEmpty()) {
            return CustomResponse
                    .create()
                    .status(HttpStatus.NOT_FOUND)
                    .build();
        }

        return CustomResponse
                .create()
                .addModel(user.get())
                .status(HttpStatus.FOUND)
                .build();
    }

    @PostMapping(
            consumes = "application/json",
            produces = "application/json"
    )
    public ResponseEntity<?> createUser(@Validated @RequestBody UserFullInfoRequest userFullInfoRequest, BindingResult bindingResult) {
        Optional<String> error;

        if (bindingResult.hasErrors()) {
            return CustomResponse
                    .create()
                    .error_message(Objects.requireNonNull(bindingResult.getFieldError()).getDefaultMessage())
                    .status(HttpStatus.BAD_REQUEST)
                    .build();
        }

        error = _userService.createUser(userFullInfoRequest);

        if (error.isPresent()) {
            return CustomResponse
                    .create()
                    .error_message(error.get())
                    .status(HttpStatus.BAD_REQUEST)
                    .build();

        }
        return CustomResponse
                .create()
                .message("User created successfully")
                .status(HttpStatus.CREATED)
                .build();
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<?> deleteUser(@PathVariable UUID id) {
        Optional<String> error;

        if (Objects.isNull(id)) {
            return CustomResponse
                    .create()
                    .error_message("id is not a valid uuid")
                    .status(HttpStatus.BAD_REQUEST)
                    .build();
        }

        error = _userService.deleteUser(id);

        if (error.isPresent()) {
            return CustomResponse
                    .create()
                    .error_message(error.get())
                    .status(HttpStatus.NOT_FOUND)
                    .build();
        }

        return CustomResponse
                .create()
                .status(HttpStatus.NO_CONTENT)
                .build();
    }

    @PutMapping
    public ResponseEntity<?> updateUser(@Validated @RequestBody UserFullInfoRequest userFullInfoRequest, BindingResult bindingResult) {
        Optional<String> error;

        if (bindingResult.hasErrors()) {
            return CustomResponse
                    .create()
                    .error_message(Objects.requireNonNull(bindingResult.getFieldError()).getDefaultMessage())
                    .status(HttpStatus.BAD_REQUEST)
                    .build();
        }

        error = _userService.updateUser(userFullInfoRequest);

        if (error.isPresent()) {
            return CustomResponse
                    .create()
                    .error_message(error.get())
                    .status(HttpStatus.BAD_REQUEST)
                    .build();
        }
            return CustomResponse
                .create()
                .message("user information updated successfully")
                .status(HttpStatus.OK)
                .build();
    }
}
