package io.exsuslabs.retefagiolimarketbackend.service.impl;

import io.exsuslabs.retefagiolimarketbackend.model.UserModel;
import io.exsuslabs.retefagiolimarketbackend.repository.UserRepository;
import io.exsuslabs.retefagiolimarketbackend.request.UserFullInfoRequest;
import io.exsuslabs.retefagiolimarketbackend.service.UserService;
import io.exsuslabs.retefagiolimarketbackend.util.RequestConverter;
import org.apache.coyote.Request;
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
        if (_userRepository.existsById(userFullInfoRequest.getId())) return Optional.of("user already exist");
        UserModel user = RequestConverter.userFromRequest(userFullInfoRequest);
        try {
            _userRepository.save(user);
        } catch (DataAccessException e) {
            return Optional.of(e.getLocalizedMessage());
        }
        return Optional.empty();
    }

    @Override
    public Optional<String> updateUser(UserFullInfoRequest userFullInfoRequest) {
        if (!_userRepository.existsById(userFullInfoRequest.getId())) return Optional.of("user does not exist");
        UserModel user = RequestConverter.userFromRequest(userFullInfoRequest);
        try {
            _userRepository.save(user);
        }catch (DataAccessException e) {
            return Optional.of(e.getLocalizedMessage());
        }


        return Optional.empty();
    }

    @Override
    public Optional<UserModel> getUser(UUID id) {
        if (!_userRepository.existsById(id)) return Optional.empty();
        try {
            return _userRepository.findById(id);
        } catch (DataAccessException e) {
            return Optional.empty();
        }
    }

    @Override
    public Optional<String> deleteUser(UUID id) {
        if (!_userRepository.existsById(id)) return Optional.of("user does not exist");
        try {
            _userRepository.deleteById(id);
        } catch (DataAccessException e) {
            return Optional.of(e.getRootCause().getLocalizedMessage());
        }
        return Optional.empty();
    }
}
