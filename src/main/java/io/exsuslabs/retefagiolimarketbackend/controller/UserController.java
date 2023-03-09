package io.exsuslabs.retefagiolimarketbackend.controller;

import io.exsuslabs.retefagiolimarketbackend.request.UserFullInfoRequest;
import io.exsuslabs.retefagiolimarketbackend.service.UserService;
import io.exsuslabs.retefagiolimarketbackend.util.CustomResponse;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.validation.BindingResult;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.Optional;

@RestController
@RequestMapping("/user")
public class UserController {

    @Autowired
    UserService _userService;

    @PostMapping(
            consumes = "application/json",
            produces = "application/json"
    )
    public ResponseEntity<?> createUser(@Validated @RequestBody UserFullInfoRequest userFullInfoRequest, BindingResult bindingResult) {
        Optional<String> error;

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
}
