package io.exsuslabs.retefagiolimarketbackend.controller;

import io.exsuslabs.retefagiolimarketbackend.model.UserModel;
import io.exsuslabs.retefagiolimarketbackend.request.UserFullInfoRequest;
import io.exsuslabs.retefagiolimarketbackend.service.UserService;
import io.exsuslabs.retefagiolimarketbackend.util.CustomResponse;
import java.util.Objects;
import java.util.Optional;
import java.util.UUID;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.validation.BindingResult;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

/**
* User Controller.
*/
@RestController
@RequestMapping("/user")
public class UserController {

  @Autowired
  UserService userService;

  /**
  *
   * @param id
   * @return
  */
  @GetMapping("/{id}")
  public ResponseEntity<?> getUser(@PathVariable UUID id) {
    Optional<UserModel> user;

    user = userService.getUser(id);

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

  /**
  *
   * @param userFullInfoRequest
   * @param bindingResult
   * @return
  */
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

    error = userService.createUser(userFullInfoRequest);

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

  /**
  *
   * @param id
   * @return
  */
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

    error = userService.deleteUser(id);

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

  /**
  *
   * @param userFullInfoRequest
   * @param bindingResult
   * @return
  */
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

    error = userService.updateUser(userFullInfoRequest);

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
