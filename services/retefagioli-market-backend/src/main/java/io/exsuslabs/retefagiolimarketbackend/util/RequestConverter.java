package io.exsuslabs.retefagiolimarketbackend.util;

import io.exsuslabs.retefagiolimarketbackend.model.UserModel;
import io.exsuslabs.retefagiolimarketbackend.request.UserFullInfoRequest;

public class RequestConverter {
    public static UserModel userFromRequest(UserFullInfoRequest userFullInfoRequest) {
        UserModel user =  new UserModel();
        user.setId(userFullInfoRequest.getId());
        user.setName(userFullInfoRequest.getName());
        user.setSurname(userFullInfoRequest.getSurname());
        user.setEmail(userFullInfoRequest.getEmail());
        user.setDob(userFullInfoRequest.getDob());
        user.setPhone(userFullInfoRequest.getPhone());
        user.setAddress(userFullInfoRequest.getAddress());
        return user;
    }
}
