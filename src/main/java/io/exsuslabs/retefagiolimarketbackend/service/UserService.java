package io.exsuslabs.retefagiolimarketbackend.service;

import io.exsuslabs.retefagiolimarketbackend.model.UserModel;
import io.exsuslabs.retefagiolimarketbackend.repository.UserRepository;
import io.exsuslabs.retefagiolimarketbackend.request.UserInfoRequest;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.dao.DataAccessException;
import org.springframework.stereotype.Service;

import java.util.Objects;
import java.util.Optional;

@Service
public class UserService {

    @Autowired
    UserRepository _userRepository;

    public Optional<String> createUser(UserInfoRequest userInfoRequest) {
        UserModel user = new UserModel();
        System.out.println(userInfoRequest.getId());
        user.setId(userInfoRequest.getId());
        user.setName(userInfoRequest.getName());
        user.setSurname(userInfoRequest.getSurname());
        user.setEmail(userInfoRequest.getEmail());
        user.setDob(userInfoRequest.getDob());
        user.setPhone(userInfoRequest.getPhone());
        user.setAddress(userInfoRequest.getAddress());
        try {
            _userRepository.save(user);
        } catch (DataAccessException e) {
            return Optional.of(e.getMessage());
        }
        return Optional.empty();
    }
}
