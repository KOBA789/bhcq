#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Code(pub u8);

impl Code {
    pub const PAD                                              : Code = Code(0);
    pub const SUBNET_MASK                                      : Code = Code(1);
    pub const TIME_OFFSET                                      : Code = Code(2);
    pub const ROUTER                                           : Code = Code(3);
    pub const TIME_SERVER                                      : Code = Code(4);
    pub const NAME_SERVER                                      : Code = Code(5);
    pub const DOMAIN_NAME_SERVER                               : Code = Code(6);
    pub const LOG_SERVER                                       : Code = Code(7);
    pub const QUOTE_SERVER                                     : Code = Code(8);
    pub const LPR_SERVER                                       : Code = Code(9);
    pub const IMPRESS_SERVER                                   : Code = Code(10);
    pub const RESOURCE_LOCATION_SERVER                         : Code = Code(11);
    pub const HOST_NAME                                        : Code = Code(12);
    pub const BOOT_FILE_SIZE                                   : Code = Code(13);
    pub const MERIT_DUMP_FILE                                  : Code = Code(14);
    pub const DOMAIN_NAME                                      : Code = Code(15);
    pub const SWAP_SERVER                                      : Code = Code(16);
    pub const ROOT_PATH                                        : Code = Code(17);
    pub const EXTENSIONS_PATH                                  : Code = Code(18);
    pub const IP_FORWARDING                                    : Code = Code(19);
    pub const NON_LOCAL_SOURCE_ROUTING                         : Code = Code(20);
    pub const POLICY_FILTER                                    : Code = Code(21);
    pub const MAXIMUM_DATAGRAM_ASSEMBLY_SIZE                   : Code = Code(22);
    pub const DEFAULT_IPTTL                                    : Code = Code(23);
    pub const PATH_MTU_AGING_TIMEOUT                           : Code = Code(24);
    pub const PATH_MTU_PLATEAU_TABLE                           : Code = Code(25);
    pub const INTERFACE_MTU                                    : Code = Code(26);
    pub const ALL_SUBNETS_ARE_LOCAL                            : Code = Code(27);
    pub const BROADCAST_ADDRESS                                : Code = Code(28);
    pub const PERFORM_MASK_DISCOVERY                           : Code = Code(29);
    pub const MASK_SUPPLIER                                    : Code = Code(30);
    pub const PERFORM_ROUTER_DISCOVERY                         : Code = Code(31);
    pub const ROUTER_SOLICITATION_ADDRESS                      : Code = Code(32);
    pub const STATIC_ROUTING_TABLE                             : Code = Code(33);
    pub const TRAILER_ENCAPSULATION                            : Code = Code(34);
    pub const ARP_CACHE_TIMEOUT                                : Code = Code(35);
    pub const ETHERNET_ENCAPSULATION                           : Code = Code(36);
    pub const DEFAUL_TCPTTL                                    : Code = Code(37);
    pub const TCP_KEEPALIVE_INTERVAL                           : Code = Code(38);
    pub const TCP_KEEPALIVE_GARBAGE                            : Code = Code(39);
    pub const NETWORK_INFORMATION_SERVICE_DOMAIN               : Code = Code(40);
    pub const NETWORK_INFORMATION_SERVERS                      : Code = Code(41);
    pub const NTP_SERVERS                                      : Code = Code(42);
    pub const VENDOR_SPECIFIC_INFORMATION                      : Code = Code(43);
    pub const NET_BIOS_OVER_TCPIP_NAME_SERVER                  : Code = Code(44);
    pub const NET_BIOS_OVER_TCPIP_DATAGRAM_DISTRIBUTION_SERVER : Code = Code(45);
    pub const NET_BIOS_OVER_TCPIP_NODE_TYPE                    : Code = Code(46);
    pub const NET_BIOS_OVER_TCPIP_SCOPE                        : Code = Code(47);
    pub const X_WINDOW_SYSTEM_FONT_SERVER                      : Code = Code(48);
    pub const X_WINDOW_SYSTEM_DISPLAY_MANGER                   : Code = Code(49);
    pub const REQUESTED_IP_ADDRESS                             : Code = Code(50);
    pub const IP_ADDRESS_LEASE_TIME                            : Code = Code(51);
    pub const OVERLOAD                                         : Code = Code(52);
    pub const DHCP_MESSAGE_TYPE                                : Code = Code(53);
    pub const SERVER_IDENTIFIER                                : Code = Code(54);
    pub const PARAMETER_REQUEST_LIST                           : Code = Code(55);
    pub const MESSAGE                                          : Code = Code(56);
    pub const MAXIMUM_DHCP_MESSAGE_SIZE                        : Code = Code(57);
    pub const RENEW_TIME_VALUE                                 : Code = Code(58);
    pub const REBINDING_TIME_VALUE                             : Code = Code(59);
    pub const CLASS_IDENTIFIER                                 : Code = Code(60);
    pub const CLIENT_IDENTIFIER                                : Code = Code(61);
    pub const NET_WARE_IP_DOMAIN_NAME                          : Code = Code(62);
    pub const NET_WARE_IP_INFORMATION                          : Code = Code(63);
    pub const NETWORK_INFORMATION_SERVICE_PLUS_DOMAIN          : Code = Code(64);
    pub const NETWORK_INFORMATION_SERVICE_PLUS_SERVERS         : Code = Code(65);
    pub const TFTP_SERVER_NAME                                 : Code = Code(66);
    pub const BOOTFILE_NAME                                    : Code = Code(67);
    pub const MOBILE_IP_HOME_AGENT                             : Code = Code(68);
    pub const SIMPLE_MAIL_TRANSPORT_PROTOCOL_SERVER            : Code = Code(69);
    pub const POST_OFFICE_PROTOCOL_SERVER                      : Code = Code(70);
    pub const NETWORK_NEWS_TRANSPORT_PROTOCOL_SERVER           : Code = Code(71);
    pub const DEFAULT_WORLD_WIDE_WEB_SERVER                    : Code = Code(72);
    pub const DEFAULT_FINGER_SERVER                            : Code = Code(73);
    pub const DEFAULT_INTERNET_RELAY_CHAT_SERVER               : Code = Code(74);
    pub const STREET_TALK_SERVER                               : Code = Code(75);
    pub const STREET_TALK_DIRECTORY_ASSISTANCE_SERVER          : Code = Code(76);
    pub const END                                              : Code = Code(255);
}