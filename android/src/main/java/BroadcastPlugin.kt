package com.plugin.broadcast

import android.app.Activity
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.os.Build
import android.util.Log
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Channel

@InvokeArg
class RegisterArgs {
  var action: String? = null
  var handler: Channel? = null
}

@InvokeArg
class UnregisterArgs {
  var action: String? = null
}

@TauriPlugin
class BroadcastPlugin(private val activity: Activity): Plugin(activity) {
    private val receivers = mutableMapOf<String, BroadcastReceiver>()
    private val TAG = "TauriBroadcast"

    @Command
    fun register(invoke: Invoke) {
        val args = invoke.parseArgs(RegisterArgs::class.java)
        val action = args.action ?: return invoke.reject("Action is required")
        val handler = args.handler ?: return invoke.reject("Handler is required")
        
        Log.i(TAG, "正在注册 Action: $action")
        
        receivers[action]?.let {
            try { activity.unregisterReceiver(it) } catch (e: Exception) {}
        }

        val receiver = object : BroadcastReceiver() {
            override fun onReceive(context: Context?, intent: Intent?) {
                intent?.let {
                    Log.i(TAG, "收到广播 Action: ${it.action}")
                    val event = JSObject()
                    event.put("action", it.action)
                    
                    val extras = JSObject()
                    it.extras?.let { bundle ->
                        for (key in bundle.keySet()) {
                            val value = bundle.get(key)
                            extras.put(key, value)
                        }
                    }
                    event.put("extras", extras)
                    
                    // 通过 Channel 发送给前端
                    handler.send(event)
                }
            }
        }

        val filter = IntentFilter(action)
        try {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                activity.registerReceiver(receiver, filter, Context.RECEIVER_EXPORTED)
            } else {
                activity.registerReceiver(receiver, filter)
            }
            receivers[action] = receiver
            Log.i(TAG, "注册成功: $action")
            invoke.resolve()
        } catch (e: Exception) {
            Log.e(TAG, "注册失败: ${e.message}")
            invoke.reject("注册失败: ${e.message}")
        }
    }

    @Command
    fun unregister(invoke: Invoke) {
        val args = invoke.parseArgs(UnregisterArgs::class.java)
        val action = args.action ?: return invoke.reject("Action is required")
        
        receivers[action]?.let {
            activity.unregisterReceiver(it)
            receivers.remove(action)
            Log.i(TAG, "已注销: $action")
        }
        invoke.resolve()
    }

    override fun onDestroy() {
        for (receiver in receivers.values) {
            try { activity.unregisterReceiver(receiver) } catch (e: Exception) {}
        }
        receivers.clear()
        super.onDestroy()
    }
}
