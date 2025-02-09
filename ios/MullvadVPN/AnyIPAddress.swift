//
//  AnyIPAddress.swift
//  MullvadVPN
//
//  Created by pronebird on 05/10/2021.
//  Copyright © 2021 Mullvad VPN AB. All rights reserved.
//

import Foundation
import Network

/// Container type that holds either `IPv4Address` or `IPv6Address`.
enum AnyIPAddress: IPAddress, Codable, Equatable, CustomDebugStringConvertible {
    case ipv4(IPv4Address)
    case ipv6(IPv6Address)

    private enum CodingKeys: String, CodingKey {
        case ipv4, ipv6
    }

    private var innerAddress: IPAddress {
        switch self {
        case .ipv4(let ipv4Address):
            return ipv4Address
        case .ipv6(let ipv6Address):
            return ipv6Address
        }
    }

    var rawValue: Data {
        return innerAddress.rawValue
    }

    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        if container.contains(.ipv4) {
            self = .ipv4(try container.decode(IPv4Address.self, forKey: .ipv4))
        } else if container.contains(.ipv6) {
            self = .ipv6(try container.decode(IPv6Address.self, forKey: .ipv6))
        } else {
            throw DecodingError.dataCorruptedError(forKey: .ipv4, in: container, debugDescription: "Invalid AnyIPAddress representation")
        }
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        switch self {
        case .ipv4(let ipv4Address):
            try container.encode(ipv4Address, forKey: .ipv4)
        case .ipv6(let ipv6Address):
            try container.encode(ipv6Address, forKey: .ipv6)
        }
    }

    init?(_ rawValue: Data, _ interface: NWInterface?) {
        if let ipv4Address = IPv4Address(rawValue, interface) {
            self = .ipv4(ipv4Address)
        } else if let ipv6Address = IPv6Address(rawValue, interface) {
            self = .ipv6(ipv6Address)
        } else {
            return nil
        }
    }

    init?(_ string: String) {
        if let ipv4Address = IPv4Address(string) {
            self = .ipv4(ipv4Address)
        } else if let ipv6Address = IPv6Address(string) {
            self = .ipv6(ipv6Address)
        } else {
            return nil
        }
    }

    var interface: NWInterface? {
        return innerAddress.interface
    }

    var isLoopback: Bool {
        return innerAddress.isLoopback
    }

    var isLinkLocal: Bool {
        return innerAddress.isLinkLocal
    }

    var isMulticast: Bool {
        return innerAddress.isMulticast
    }

    var debugDescription: String {
        switch self {
        case .ipv4(let ipv4Address):
            return "\(ipv4Address)"
        case .ipv6(let ipv6Address):
            return "\(ipv6Address)"
        }
    }
}
