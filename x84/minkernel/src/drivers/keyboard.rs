// keyboard driver for usb keyboard


use crate::usb::usb_device::UsbDevice;
use crate::usb::usb_device::UsbDeviceState;
use crate::usb::usb_device::UsbDeviceType;
use crate::usb::usb_device::UsbDeviceRequest;
use crate::usb::usb_device::UsbDeviceRequestType;
use crate::usb::usb_device::UsbDeviceRequestRecipient;
use crate::usb::usb_device::UsbDeviceRequestDirection;
use crate::usb::usb_device::UsbDeviceRequestCode;
use crate::usb::usb_device::UsbDeviceRequestStatus;
use crate::usb::usb_device::UsbDeviceRequestData;
use crate::usb::usb_device::UsbDeviceRequestDataDirection;
use crate::usb::usb_device::UsbDeviceRequestDataStatus;
use crate::usb::usb_device::UsbDeviceRequestDataValue;
use crate::usb::usb_device::UsbDeviceRequestDataIndex;
use crate::usb::usb_device::UsbDeviceRequestDataLength;
use crate::usb::usb_device::UsbDeviceRequestDataBuffer;
use crate::usb::usb_device::UsbDeviceRequestDataBufferLength;
use crate::usb::usb_device::UsbDeviceRequestDataBufferIndex;
use crate::usb::usb_device::UsbDeviceRequestDataBufferStatus;
use crate::usb::usb_device::UsbDeviceRequestDataBufferDirection;
use crate::usb::usb_device::UsbDeviceRequestDataBufferValue;
use crate::usb::usb_device::UsbDeviceRequestDataBufferIndexValue;
use crate::usb::usb_device::UsbDeviceRequestDataBufferLengthValue;
use crate::usb::usb_device::UsbDeviceRequestDataBufferStatusValue;
use crate::usb::usb_device::UsbDeviceRequestDataBufferDirectionValue;



fn main() {
    let mut input = [0u8; 8];
    let mut usb_device = UsbDevice::new(UsbDeviceType::Keyboard, UsbDeviceState::Connected, input);


    if usb_device.get_state() == UsbDeviceState::Connected {
        let mut request = UsbDeviceRequest::new(UsbDeviceRequestType::Standard, UsbDeviceRequestRecipient::Device, UsbDeviceRequestDirection::HostToDevice, UsbDeviceRequestCode::SetAddress, UsbDeviceRequestStatus::Idle);
        let mut data = UsbDeviceRequestData::new(UsbDeviceRequestDataDirection::HostToDevice, UsbDeviceRequestDataStatus::Idle, UsbDeviceRequestDataValue::Address(0x01), UsbDeviceRequestDataIndex::Address(0x01), UsbDeviceRequestDataLength::Address(0x01));
        let mut buffer = UsbDeviceRequestDataBuffer::new(UsbDeviceRequestDataBufferDirectionValue::HostToDevice, UsbDeviceRequestDataBufferStatusValue::Idle, UsbDeviceRequestDataBufferIndexValue::Address(0x01), UsbDeviceRequestDataBufferLengthValue::Address(0x01));
        buffer.set_buffer(&mut input);
        data.set_buffer(&mut buffer);
        request.set_data(&mut data);
        usb_device.set_request(&mut request);
    }
    else {
        println!("Error: Device not connected");
    }

}
fn input_includer() {
    let mut verify = usb_device.get_input().verify();

    unsafe {
        if verify == true {
            let mut input = usb_device.get_input().get_input();
            let mut request = UsbDeviceRequest::new(UsbDeviceRequestType::Standard, UsbDeviceRequestRecipient::Device, UsbDeviceRequestDirection::HostToDevice, UsbDeviceRequestCode::SetAddress, UsbDeviceRequestStatus::Idle);
            let mut data = UsbDeviceRequestData::new(UsbDeviceRequestDataDirection::HostToDevice, UsbDeviceRequestDataStatus::Idle, UsbDeviceRequestDataValue::Address(0x01), UsbDeviceRequestDataIndex::Address(0x01), UsbDeviceRequestDataLength::Address(0x01));
            let mut buffer = UsbDeviceRequestDataBuffer::new(UsbDeviceRequestDataBufferDirectionValue::HostToDevice, UsbDeviceRequestDataBufferStatusValue::Idle, UsbDeviceRequestDataBufferIndexValue::Address(0x01), UsbDeviceRequestDataBufferLengthValue::Address(0x01));
            buffer.set_buffer(&mut input);
            data.set_buffer(&mut buffer);
            request.set_data(&mut data);
            usb_device.set_request(&mut request);
        }
        else {
            println!("Error: Input not verified");
        }
    }
    
    let data = usb_device.get_request().get_data();
    let buffer = data.get_buffer();
    let input = buffer.get_buffer();
    
    if input == {
        "a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, ";
        "Enter, Esc, Backspace, Tab, Space, -_, =+, [{, ]}, \, |, ;:, ',  , `, /?, .>, ,<,"
        '"';
        println!(input) -> sync;
    }

    else {
        println!("Error: Invalid input");
    }
}
