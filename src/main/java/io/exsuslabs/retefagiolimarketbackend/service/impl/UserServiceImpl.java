package io.exsuslabs.retefagiolimarketbackend.service.impl;

import io.exsuslabs.retefagiolimarketbackend.model.UserModel;
import io.exsuslabs.retefagiolimarketbackend.repository.UserRepository;
import io.exsuslabs.retefagiolimarketbackend.request.UserFullInfoRequest;
import io.exsuslabs.retefagiolimarketbackend.service.UserService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.dao.DataAccessException;
import org.springframework.stereotype.Service;

import java.util.Optional;
import java.util.UUID;

@Service
public class UserServiceImpl implements UserService {

    @Autowired
    UserRepository _userRepository;

    public Optional<String> createUser(UserFullInfoRequest userFullInfoRequest) {
        UserModel user = new UserModel();
        System.out.println(userFullInfoRequest.getId());
        user.setId(userFullInfoRequest.getId());
        user.setName(userFullInfoRequest.getName());
        user.setSurname(userFullInfoRequest.getSurname());
        user.setEmail(userFullInfoRequest.getEmail());
        user.setDob(userFullInfoRequest.getDob());
        user.setPhone(userFullInfoRequest.getPhone());
        user.setAddress(userFullInfoRequest.getAddress());
        try {
            _userRepository.save(user);
        } catch (DataAccessException e) {
            return Optional.of(e.getMessage());
        }
        return Optional.empty();
    }

    @Override
    public Optional<String> updateUser(UserFullInfoRequest userFullInfoRequest) {
        return Optional.empty();
    }

    @Override
    public Optional<String> getUser(UUID id) {
        return Optional.empty();
    }

    @Override
    public Optional<String> deleteUser(UUID id) {
        return Optional.empty();
    }
}
