# frozen_string_literal: true

Vagrant.configure('2') do |config|
  ENV['LC_ALL'] = 'en_US.UTF-8'
  config.vm.box = 'gusztavvargadr/windows-11'
  config.vm.box_version = '2601.0.0'

  config.vm.boot_timeout = 1000
  config.vm.hostname = 'host'

  config.vm.network 'private_network', type: 'dhcp'
  config.vm.synced_folder '.', '/vagrant', disable: true
  config.vm.synced_folder '.', '/vagrant', type: 'rsync', disabled: true

  config.vm.provider :libvirt do |libvirt|
    libvirt.memory = '8192'
    libvirt.cpus = '4'
    libvirt.default_prefix = 'Windows11'
    libvirt.storage_pool_name = 'default'
    libvirt.qemu_use_session = false
    libvirt.keymap = 'en-us'
    libvirt.graphics_type = 'vnc'
    libvirt.video_type = 'vga'
  end

  config.vm.provider 'virtualbox' do |vb|
    vb.memory = '8192'
    vb.cpus = '4'
    vb.name = 'Windows11'
    vb.gui = false
    vb.check_guest_additions = false
    vb.customize ['modifyvm', :id, '--clipboard', 'bidirectional']
  end

  config.vm.provider 'vmware_desktop' do |vmware|
    vmware.memory = '8192'
    vmware.cpus = '4'
    vmware.gui = false
    vmware.utility_certificate_path = '/opt/vagrant-vmware-desktop/certificates'
  end

  config.vm.provision 'shell', inline: <<-POWERSHELL
    powershell.exe -ExecutionPolicy Bypass -Command "winget settings --enable BypassCertificatePinningForMicrosoftStore"
    powershell.exe -ExecutionPolicy Bypass -Command "winget install --id Git.Git -e --source winget"
    powershell.exe -ExecutionPolicy Bypass -Command "winget install --id MSYS2.MSYS2 -e --source winget"
    powershell.exe -ExecutionPolicy Bypass -Command "winget install --id Rustlang.Rustup --accept-source-agreements -e --source winget"
    powershell.exe -ExecutionPolicy Bypass -Command "rustup toolchain install nightly-x86_64-pc-windows-gnu"
    powershell.exe -ExecutionPolicy Bypass -Command "rustup default nightly-x86_64-pc-windows-gnu"
    powershell.exe -ExecutionPolicy Bypass -Command "git clone https://github.com/MikeHorn-git/Undine.git"
    C:/msys64/usr/bin/bash -lc "pacman -S mingw-w64-x86_64-toolchain --needed --noconfirm"
  POWERSHELL
end
