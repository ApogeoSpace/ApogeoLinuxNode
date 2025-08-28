DEVNAME := iotd0
BOOT_DELAY := 1min
INTERVAL := 3min

INSTALL_PREFIX := /usr/local
SYSTEMD_INSTALL_DIR := /etc/systemd/system

ETC_DIR := $(INSTALL_PREFIX)/etc/iotd
BIN_DIR := $(INSTALL_PREFIX)/bin/iotd
VAR_DIR := $(INSTALL_PREFIX)/var

BIN_FILES_WILDCARD = $(wildcard bin/*)
CONF_FILES_WILDCARD = $(wildcard etc/iotd/*)
BIN_FILES = $(BIN_FILES_WILDCARD:bin/%=%)
CONF_FILES = $(CONF_FILES_WILDCARD:etc/iotd/%=%)
DATA_FILE := $(VAR_DIR)/$(DEVNAME)


$(SYSTEMD_INSTALL_DIR) $(BIN_DIR) $(ETC_DIR)/$(DEVNAME) $(VAR_DIR):
	@echo "[DIR] $@"
	@mkdir -p $@
	@chmod 755 $@

$(SYSTEMD_INSTALL_DIR)/%: % $(SYSTEMD_INSTALL_DIR)
	@echo "[SYSTEMD] $< => $@"
	@cp $< $@
	@chmod 644 $@

$(SYSTEMD_INSTALL_DIR)/%.timer: iotdnode.timer $(SYSTEMD_INSTALL_DIR)
	@echo "[SYSTEMD] $< => $@"
	@cp $< $@
	@chmod 644 $@

$(BIN_DIR)/%: bin/% $(BIN_DIR)
	@echo "[BIN] $< => $@"
	@cp $< $@
	@chmod 755 $@

$(ETC_DIR)/$(DEVNAME)/%: etc/iotd/% $(ETC_DIR)/$(DEVNAME)
	@echo "[CONF] $< => $@"
	@cp $< $@
	@chmod 600 $@

$(DATA_FILE): $(VAR_DIR)
	@dd if=/dev/zero of="$@" bs=1 count=10
	@echo DATAFILE created

configure: $(SYSTEMD_INSTALL_DIR)/iotdconfig@.service $(SYSTEMD_INSTALL_DIR)/iotdnode@.service $(SYSTEMD_INSTALL_DIR)/iotdnode@$(DEVNAME).timer $(DATA_FILE)
	sed -i 's#%BINDIR%#$(BIN_DIR)#g' $(SYSTEMD_INSTALL_DIR)/iotdconfig@.service
	sed -i 's#%CONFDIR%#$(ETC_DIR)#g' $(SYSTEMD_INSTALL_DIR)/iotdconfig@.service
	
	sed -i 's#%BINDIR%#$(BIN_DIR)#g' $(SYSTEMD_INSTALL_DIR)/iotdnode@.service
	sed -i 's#%DATAFILE%#$(DATA_FILE)#g' $(SYSTEMD_INSTALL_DIR)/iotdnode@.service
	
	sed -i 's#%BINDIR%#$(BIN_DIR)#g' $(SYSTEMD_INSTALL_DIR)/iotdnode@$(DEVNAME).timer
	sed -i 's#%BOOTTIME%#$(BOOT_DELAY)#g' $(SYSTEMD_INSTALL_DIR)/iotdnode@$(DEVNAME).timer
	sed -i 's#%INTERVAL%#$(INTERVAL)#g' $(SYSTEMD_INSTALL_DIR)/iotdnode@$(DEVNAME).timer

enable:
	systemctl daemon-reload
	systemctl enable iotdconfig@$(DEVNAME).service
	systemctl enable iotdnode@$(DEVNAME).timer

disable:
	systemctl disable iotdconfig@$(DEVNAME).service
	systemctl disable iotdnode@$(DEVNAME).timer
	systemctl stop iotdconfig@$(DEVNAME).service
	systemctl stop iotdnode@$(DEVNAME).timer

start:
	systemctl start iotdconfig@$(DEVNAME).service
	systemctl start iotdnode@$(DEVNAME).timer

install: configure $(BIN_FILES:%=$(BIN_DIR)/%) $(CONF_FILES:%=$(ETC_DIR)/$(DEVNAME)/%)

uninstall:
	rm $(SYSTEMD_INSTALL_DIR)/iotdconfig@.service
	rm $(SYSTEMD_INSTALL_DIR)/iotdnode@.service
	rm $(SYSTEMD_INSTALL_DIR)/iotdnode@*.timer
	rm $(BIN_FILES:%=$(BIN_DIR)/%)
	rm -r $(ETC_DIR)/$(DEVNAME)
	rm $(DATA_FILE)

.PHONY: install configure enable start