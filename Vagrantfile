# -*- mode: ruby -*-
# vi: set ft=ruby ts=2 sw=2 expandtab :

PROJECT = "paris-street-path-finder"

DOCKER_ENV = {
  "HOST_USER_UID" => Process.euid
}

ENV['VAGRANT_NO_PARALLEL'] = 'yes'
ENV['VAGRANT_DEFAULT_PROVIDER'] = 'docker'
Vagrant.configure(2) do |config|

  config.ssh.insert_key = false
  config.vm.define "dev", primary: true do |app|
    app.vm.network "forwarded_port", guest: 9090, host: 9090
    app.vm.provider "docker" do |d|
      d.image = "jean553/rust-dev-docker"
      d.name = "#{PROJECT}_dev"
      d.has_ssh = true
      d.env = DOCKER_ENV
    end
    app.ssh.username = "vagrant"

    app.vm.provision "add_help", type: "shell" do |s|
      zshrc = <<~EORUBY
        echo "Build:"
        echo "  $fg[green]cargo build --release$fg[white]"
        echo "Start:"
        echo "  $fg[green]./target/release/paris-street-path-finder$fg[white]"
        cd /vagrant
      EORUBY
      s.inline = "echo '#{zshrc}' >> /home/vagrant/.zshrc"
    end
  end
end
