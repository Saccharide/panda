{%- for arch, syscalls in syscalls_arch|dictsort -%}
#if {{architectures[arch].get('boilerplate_target', architectures[arch].qemu_target)}}
{%- for syscall_name, syscall in syscalls|dictsort %}
PPP_CB_BOILERPLATE(on_{{syscall.name}}_enter)
{%- endfor %}
#endif
{% endfor %}
PPP_CB_BOILERPLATE(on_unknown_sys_enter)
PPP_CB_BOILERPLATE(on_all_sys_enter)
PPP_CB_BOILERPLATE(on_all_sys_enter2)

/* vim: set tabstop=4 softtabstop=4 noexpandtab ft=cpp: */
