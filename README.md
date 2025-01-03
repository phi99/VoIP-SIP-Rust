***In Progress**

	 	---------------------------------------------------------------------------------------------------------
	     	  ** Container Orchestrator Kubernetes (K8s) Internals and implementing basic Container orchestrator **
	  	---------------------------------------------------------------------------------------------------------


```text
Container Orchestrator such as Kubernetes, etc, are basically system that manages containers such as deploying, scaling , etc and also provide automation features.
Its basic components are generally task, job, scheduler, manager, worker, cluster, CLI. Scheduler, manager, and worker are fundamental components that operate on tasks.
```

```text
Basic Orchestrator Component and Flow Overview
-------------------------------------------------
[Manager:Store/Queue task, Maintain list of workers]--(Scheduling worker and communicate via API to Worker and User)-->[Worker: Run task, Store task, Calculate Metrics]
    			/|\
    			 |
[Scheduler: Feasibility, Scoring, and pick which worker should manager place a task]

```text
Task Implementation
----------------------

main.go
-----------------------------------------------------------------------------------------------------------------------
package main

import (
	"testclient2/task"
	"fmt"
	"time"
	"context"
	"github.com/docker/docker/client"
	//"github.com/golang-collections/collections/queue"
	"github.com/google/uuid"

	//"testprojectc3/manager"
	//"testprojectc3/worker"
)

func createContainer() (*task.Docker, *task.DockerResult) {
	c := task.Config{
		Name:  "testcontainer1",
		Image: "postgres:13",
		Env: []string{
			"POSTGRES_USER=testuser",
			"POSTGRES_PASSWORD=testsecret",
		},
	}
	ctx := context.Background()
	dc, _ := client.NewClientWithOpts(client.FromEnv)
	dc.NegotiateAPIVersion(ctx)
	d := task.Docker{
		Client: dc,
		Config: c,
	}

	result := d.Run()
	if result.Error != nil {
		fmt.Printf("%v\n", result.Error)
		return nil, nil
	}

	fmt.Printf("Container %s is running with config %v\n", result.ContainerId, c)
	return &d, &result
}

func stopContainer(d *task.Docker, id string) *task.DockerResult {
	result := d.Stop(id)
	if result.Error != nil {
		fmt.Printf("%v\n", result.Error)
		return nil
	}

	fmt.Printf("Container %s has been stopped and removed\n", result.ContainerId)
	return &result
}

func main() {
	t := task.Task{
		ID:     uuid.New(),
		Name:   "Task-1",
		State:  task.Pending,
		Image:  "Image-1",
		Memory: 1024,
		Disk:   1,
	}

	te := task.TaskEvent{
		ID:        uuid.New(),
		State:     task.Pending,
		Timestamp: time.Now(),
		Task:      t,
	}

	fmt.Printf("task: %v\n", t)
	fmt.Printf("task event: %v\n", te)

	fmt.Printf("create a test container\n")
	dockerTask, createResult := createContainer()

	time.Sleep(time.Second * 5)

	fmt.Printf("stopping container %s\n", createResult.ContainerId)
	_ = stopContainer(dockerTask, createResult.ContainerId)
}
--------------------------------------------------------------------------------------------------------------------------------------------------------------

task.go
--------------------------------------------------------------------------------------------------------------------------------------------------
package task


import (
	"io"
	"log"
	"os"
	"time"
	"context"
	//"github.com/docker/docker/api/types"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/docker/api/types/image"
	"github.com/docker/docker/client"
	"github.com/docker/go-connections/nat"
	"github.com/google/uuid"
	"github.com/moby/moby/pkg/stdcopy"
)

type State int

const (
    Pending State = iota
    Scheduled
    Running
    Completed
    Failed
)

type Task struct {
	ID            uuid.UUID
	ContainerID   string
	Name          string
	State         State
	Image         string
	Cpu           float64
	Memory        int64
	Disk          int64
	ExposedPorts  nat.PortSet
	PortBindings  map[string]string
	RestartPolicy container.RestartPolicyMode
	StartTime     time.Time
	FinishTime    time.Time
}

type TaskEvent struct {
	ID        uuid.UUID
	State     State
	Timestamp time.Time
	Task      Task
}

// Config struct to hold Docker container config
type Config struct {
	// Name of the task, also used as the container name
	Name string
	// AttachStdin boolean which determines if stdin should be attached
	AttachStdin bool
	// AttachStdout boolean which determines if stdout should be attached
	AttachStdout bool
	// AttachStderr boolean which determines if stderr should be attached
	AttachStderr bool
	// ExposedPorts list of ports exposed
	ExposedPorts nat.PortSet
	// Cmd to be run inside container (optional)
	Cmd []string
	// Image used to run the container
	Image string
	// Cpu
	Cpu float64
	// Memory in MiB
	Memory int64
	// Disk in GiB
	Disk int64
	// Env variables
	Env []string
	// RestartPolicy for the container ["", "always", "unless-stopped", "on-failure"]
	RestartPolicy container.RestartPolicyMode
}

func NewConfig(t *Task) *Config {
	return &Config{
		Name:          t.Name,
		ExposedPorts:  t.ExposedPorts,
		Image:         t.Image,
		Cpu:           t.Cpu,
		Memory:        t.Memory,
		Disk:          t.Disk,
		RestartPolicy: t.RestartPolicy,
	}
}

type Docker struct {
	Client *client.Client
	Config Config
}

func NewDocker(c *Config) *Docker {
	dc, _ := client.NewClientWithOpts(client.FromEnv)
	return &Docker{
		Client: dc,
		Config: *c,
	}
}

type DockerResult struct {
	Error       error
	Action      string
	ContainerId string
	Result      string
}

func (d *Docker) Run() DockerResult {
	ctx := context.Background()
	reader, err := d.Client.ImagePull(ctx, d.Config.Image, image.PullOptions{})
	//reader, err := d.Client.ImagePull(ctx, d.Config.Image, types.ImagePullOptions{})
	//reader, err := d.Client.ImagePull(ctx, d.Config.Image, d.Client.Image.PullOptions{})
	if err != nil {
		log.Printf("Error pulling image %s: %v\n", d.Config.Image, err)
		return DockerResult{Error: err}
	}
	io.Copy(os.Stdout, reader)

	rp := container.RestartPolicy{
		Name: d.Config.RestartPolicy,
	}

	r := container.Resources{
		Memory: d.Config.Memory,
	}

	cc := container.Config{
		Image:        d.Config.Image,
		Tty:          false,
		Env:          d.Config.Env,
		ExposedPorts: d.Config.ExposedPorts,
	}

	hc := container.HostConfig{
		RestartPolicy:   rp,
		Resources:       r,
		PublishAllPorts: true,
	}

	resp, err := d.Client.ContainerCreate(ctx, &cc, &hc, nil, nil, d.Config.Name)
	if err != nil {
		log.Printf("Error creating container using image %s: %v\n", d.Config.Image, err)
		return DockerResult{Error: err}
	}

	if err = d.Client.ContainerStart(ctx, resp.ID, container.StartOptions{}); err != nil {
		log.Printf("Error starting container %s: %v\n", resp.ID, err)
		return DockerResult{Error: err}
	}

	out, err := d.Client.ContainerLogs(ctx, resp.ID, container.LogsOptions{ShowStdout: true, ShowStderr: true})
	if err != nil {
		log.Printf("Error getting logs for container %s: %v\n", resp.ID, err)
		return DockerResult{Error: err}
	}

	stdcopy.StdCopy(os.Stdout, os.Stderr, out)

	return DockerResult{ContainerId: resp.ID, Action: "start", Result: "success"}
}

func (d *Docker) Stop(id string) DockerResult {
	log.Printf("Attempting to stop container %v", id)
	ctx := context.Background()
	err := d.Client.ContainerStop(ctx, id, container.StopOptions{})
	if err != nil {
		log.Printf("Error stopping container %s: %v\n", id, err)
		return DockerResult{Error: err}
	}

	err = d.Client.ContainerRemove(ctx, id, container.RemoveOptions{
		RemoveVolumes: true,
		RemoveLinks:   false,
		Force:         false,
	})
	if err != nil {
		log.Printf("Error removing container %s: %v\n", id, err)
		return DockerResult{Error: err}
	}

	return DockerResult{Action: "stop", Result: "success", Error: nil}
}

-------------------------------------------------------------------------------------------------------------------------------------------------------


```









